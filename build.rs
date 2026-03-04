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

    // bundle the whamm_core lib
    bundle_whamm_core(&out_dir);

    // -- bundle the provider definitions
    let defs_dir = "./";
    bundle_defs(defs_dir, &mut out_file, "DEF_YAMLS");

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

fn bundle_whamm_core(out_dir: &str) {
    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-wasip1",
            "-p",
            "whamm_core",
        ])
        .status()
        .expect("Failed to build whamm_core");

    assert!(status.success());

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());

    let wasm_src = format!("{target_dir}/wasm32-wasip1/release/whamm_core.wasm");

    let wasm_dst = std::path::Path::new(out_dir).join("whamm_core.wasm");

    std::fs::copy(&wasm_src, &wasm_dst).expect("Failed to copy wasm");

    println!("cargo:rerun-if-changed=whamm_core/src/");
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
