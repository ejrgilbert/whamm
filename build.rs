use clap::CommandFactory;
use clap_mangen::Man;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Bundling files as resources in Whamm for library usage!

    let out_path = Path::new(&out_dir).join("bundled.rs");
    let mut out_file = File::create(&out_path).unwrap();

    // -- bundle the provider definitions
    let defs_dir = "./";
    bundle_defs(defs_dir, &mut out_file, "DEF_YAMLS");

    // -- bundle the whamm_core library
    let whamm_core_path_module = "whamm_core-module/target/wasm32-wasip1/release/whamm_core.wasm";
    let whamm_core_path_component = "whamm_core-component/target/wasm32-wasip2/release/whamm_core.wasm";
    bundle_wasm(
        whamm_core_path_module,
        build_core_library_module,
        &mut out_file,
        "WHAMM_CORE_LIB_BYTES_MODULE",
    );
    bundle_wasm(
        whamm_core_path_component,
        build_core_library_component,
        &mut out_file,
        "WHAMM_CORE_LIB_BYTES_COMPONENT",
    );

    // Build the CLI manual.

    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    // Create `target/assets/` folder.
    let cli_out_path = Path::new(&out_dir).join("assets");
    fs::create_dir_all(&cli_out_path).unwrap();

    build_man(&cli_out_path)?;

    Ok(())
}

// ================================
// ====== Bundling Resources ======
// ================================

include!("src/parser/yml_processor.rs");

fn bundle_defs(base_dir: &str, out_file: &mut File, var_name: &str) {
    let defs = pull_all_yml_files(base_dir);
    writeln!(out_file, "pub static {var_name}: &[&str] = &[").unwrap();
    for def in defs.iter() {
        writeln!(out_file, "    {:?},", def).unwrap();
    }
    writeln!(out_file, "];").unwrap();
}

fn bundle_wasm(p: &str, build_wasm: fn(), out_file: &mut File, wasm_var_name: &str) {
    // ALWAYS build it -- ensures wasm is up to date
    build_wasm();

    let data = fs::read(p).unwrap_or_else(|_| panic!("Failed to read Wasm binary: {}", p));
    write!(out_file, "pub static {wasm_var_name}: &[u8] = &[").unwrap();
    for byte in data {
        write!(out_file, "{},", byte).unwrap();
    }
    writeln!(out_file, "];").unwrap();
}

fn build_core_library_module() {
    build_core_library("whamm_core-module", "wasm32-wasip1");
}

fn build_core_library_component() {
    build_core_library("whamm_core-component", "wasm32-wasip2");
}

fn build_core_library(dir: &str, target: &str) {
    let res = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(target)
        .arg("--release")
        .current_dir(dir)
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        println!(
            "[ERROR] 'whamm_core' build project failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
    assert!(res.status.success());
}

// ==================================
// ====== Build the CLI Manual ======
// ==================================

include!("src/cli.rs");

fn build_man(out_dir: &Path) -> Result<(), Error> {
    let app = WhammCli::command();

    let file = Path::new(&out_dir).join("example.1");
    let mut file = File::create(file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}
