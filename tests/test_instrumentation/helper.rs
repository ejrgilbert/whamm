use crate::util::{setup_logger, DEFAULT_CORE_LIB_PATH_COMPONENT, DEFAULT_CORE_LIB_PATH_MODULE};
use glob::{glob, glob_with};
use log::{error, warn};
use serde::de::Expected;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use wac_graph::types::Package;
use wac_graph::{CompositionGraph, EncodeOptions};
use whamm::api::instrument::{instrument_as_dry_run_rewriting, WhammError};
use whamm::api::utils::{wasm2wat_on_file, write_to_file};
use wirm::Module;

pub const DEFAULT_DEFS_PATH: &str = "./";
const TEST_RSC_DIR: &str = "tests/scripts/";
const MM_PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

// FOR COMPONENTS
const RUN_FUNC_PREFIX: &str = "wasi:cli/run@";

fn get_test_scripts(sub_dir: &str) -> Vec<(PathBuf, String)> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*MM_PATTERN.to_owned()))
        .expect("Failed to read glob pattern")
    {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        scripts.push((file_name.clone(), unparsed_file));
    }

    for path in glob_with(
        &(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*TODO.to_owned()),
        options,
    )
    .expect("Failed to read glob pattern")
    {
        warn!(
            "File marked with TODO: {}",
            path.as_ref().unwrap().display()
        );
    }

    scripts
}

pub fn run_whamm_bin(
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
    defs_path: &str,
    core_lib_path: &str,
) {
    // executable is located at target/debug/whamm
    let executable = "target/debug/whamm";

    let res = Command::new(executable)
        .arg("instr")
        .arg("--script")
        .arg(monitor_path)
        .arg("--app")
        .arg(original_wasm_path)
        .arg("--output-path")
        .arg(instrumented_wasm_path)
        .arg("--defs-path")
        .arg(defs_path)
        .arg("--core-lib")
        .arg(core_lib_path)
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'run_whamm_bin' add target failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
    assert!(res.status.success());
}

pub fn run_basic_instrumentation(
    original_wat_path: &str,
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
) {
    wat2wasm_on_file(original_wat_path, original_wasm_path);
    run_whamm_bin(
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
        DEFAULT_DEFS_PATH,
        DEFAULT_CORE_LIB_PATH_MODULE,
    );
    wasm2wat_on_file(instrumented_wasm_path);
}

pub fn wat2wasm_on_file(original_wat_path: &str, original_wasm_path: &str) {
    let res = Command::new("wasm-tools")
        .arg("parse")
        .arg(original_wat_path)
        .arg("-o")
        .arg(original_wasm_path)
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'wasm-tools parse' failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
}

pub fn setup_fault_injection(variation: &str) -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts(format!("fault_injection/{variation}").as_str());
    if scripts.is_empty() {
        warn!("No test scripts found for `fault_injection/{variation}` test.");
    }

    scripts
}

pub fn setup_wizard_monitors() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.is_empty() {
        warn!("No test scripts found for `wizard_monitors` test.");
    }

    scripts
}

pub fn setup_replay() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.is_empty() {
        warn!("No test scripts found for `replay` test.");
    }

    scripts
}

pub fn setup_numerics_monitors() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("core_suite/numerics");
    if scripts.is_empty() {
        warn!("No test scripts found for `numerics` test.");
    }

    scripts
}

pub fn setup_tests(dir_name: &str) -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts(dir_name);
    if scripts.is_empty() {
        warn!("No test scripts found for `{dir_name}` test.");
    }

    scripts
}

pub(crate) fn run_core_suite(
    suite_name: &str,
    processed_scripts: Vec<(PathBuf, String)>,
    as_component: bool,
    with_br: bool,
    with_wei: bool,
    dry_run: bool,
) {
    let mut rewriting_tests = vec![];
    let mut wei_tests = vec![];
    for (script_path, ..) in processed_scripts.iter() {
        let fname = script_path.file_name().unwrap().to_str().unwrap();
        let path = script_path.parent().unwrap();

        let app = path.join("app").join(format!("{}.app", fname));
        let libs = path.join("libs").join(format!("{}.libs", fname));
        let rewriting_exp = path
            .join("expected")
            .join("rewriting")
            .join(format!("{}.exp", fname));
        let wei_exp = path
            .join("expected")
            .join("wei")
            .join(format!("{}.exp", fname));

        rewriting_tests.push(TestCase {
            script: script_path.clone(),
            app: app.clone(),
            libs: libs.clone(),
            exp: rewriting_exp,
        });
        wei_tests.push(TestCase {
            script: script_path.clone(),
            app,
            libs,
            exp: wei_exp,
        });
    }

    let outdir = format!("output/tests/{suite_name}");
    try_path(&outdir);
    let instr_app_path = format!("{outdir}/output.wasm");

    let core_lib_path = if as_component {
        DEFAULT_CORE_LIB_PATH_COMPONENT.to_string()
    } else {
        DEFAULT_CORE_LIB_PATH_MODULE.to_string()
    };

    if with_br {
        for TestCase {
            script,
            app,
            libs,
            exp,
            ..
        } in rewriting_tests.iter()
        {
            println!(
                "[REWRITE] Running test case with monitor at the following path: {:#?}",
                script
            );
            let app_path_str = fs::read_to_string(app)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", app));
            let libs_path_str = if let Ok(res) = fs::read_to_string(libs) {
                let mut libs = vec![];
                for lib in res.split('\n') {
                    libs.push(lib.to_string());
                }
                libs
            } else {
                vec![]
            };
            let metadata = fs::metadata(exp).expect("Failed to load expected output file metadata");
            let exp_out = if metadata.len() > MAX_EXP_OUT_SIZE {
                ExpectedOutput::hash(exp)
            } else {
                ExpectedOutput::Str(
                    fs::read_to_string(exp)
                        .unwrap_or_else(|_| panic!("Unable to read file at {:?}", exp)),
                )
            };
            run_testcase_rewriting(
                script,
                &app_path_str,
                libs_path_str,
                core_lib_path.clone(),
                exp_out,
                &outdir,
                &instr_app_path,
                dry_run,
                as_component,
            );
        }
    }

    if with_wei {
        for TestCase {
            script,
            app,
            libs,
            exp,
            ..
        } in wei_tests.iter()
        {
            println!(
                "[WEI] Running test case with monitor at the following path: {:#?}",
                script
            );
            let app_path_str = fs::read_to_string(app)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", app));
            let libs_path_str = if let Ok(res) = fs::read_to_string(libs) {
                let mut libs = vec![];
                for lib in res.split('\n') {
                    libs.push(lib.to_string());
                }
                libs
            } else {
                vec![]
            };
            let metadata = fs::metadata(exp).unwrap_or_else(|_| {
                panic!("Failed to load expected output file metadata at: {:?}", exp)
            });
            let exp_out = if metadata.len() > MAX_EXP_OUT_SIZE {
                ExpectedOutput::hash(exp)
            } else {
                ExpectedOutput::Str(
                    fs::read_to_string(exp)
                        .unwrap_or_else(|_| panic!("Unable to read file at {:?}", exp)),
                )
            };
            run_testcase_wei(
                script,
                &app_path_str,
                libs_path_str,
                core_lib_path.clone(),
                exp_out,
                &outdir,
                &instr_app_path,
                dry_run,
                as_component,
            );
        }
    }
}
const MAX_EXP_OUT_SIZE: u64 = 50_000; // 50 KB
enum ExpectedOutput {
    Hash(String),
    Str(String),
}
impl ExpectedOutput {
    pub fn hash(file: &PathBuf) -> Self {
        Self::Hash(file_hash(file))
    }
}

fn file_hash(file: &PathBuf) -> String {
    let res = Command::new("sha1sum")
        .arg(file)
        .output()
        .expect("failed to run sha1sum");
    if !res.status.success() {
        panic!("Could not get hash for file: {:?}", file)
    } else {
        let stdout = String::from_utf8(res.stdout).unwrap();
        let parts: Vec<&str> = stdout.split(' ').collect();
        parts[0].to_string()
    }
}

/// create output path if it doesn't exist
pub(crate) fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

pub(crate) fn run_script(
    script_path: &Path,
    wasm_path: &str,
    target_wasm_bytes: Vec<u8>,
    user_libs: Vec<String>,
    core_wasm_path: String,
    output_path: Option<String>,
    target_wei: bool,
    dry_run: bool,
) -> Result<(), Vec<WhammError>> {
    let script_path_str = script_path.to_str().unwrap().replace("\"", "");
    let wasm_result = if target_wei {
        whamm::api::instrument::generate_monitor_module(
            script_path_str,
            user_libs.clone(),
            Some(core_wasm_path.clone()),
            Some("./".to_string()),
        )
    } else {
        whamm::api::instrument::instrument_bytes_with_rewriting(
            target_wasm_bytes,
            script_path_str,
            user_libs.clone(),
            Some(core_wasm_path.clone()),
            Some("./".to_string()),
        )
    }?;
    if dry_run && !target_wei {
        let _side_effects = instrument_as_dry_run_rewriting(
            wasm_path.to_string(),
            script_path.to_str().unwrap().to_string(),
            user_libs,
            Some(core_wasm_path),
            Some("./".to_string()),
        )
        .expect("Failed to run dry-run");

        // NOTE: uncomment to debug side effects...just don't commit this uncommented! it'll slow EVERYTHING down
        // print_side_effects(&_side_effects);
    }
    if let Some(path) = output_path {
        write_to_file(wasm_result, path);
    }
    Ok(())
}

fn run_testcase_rewriting(
    script: &Path,
    app_path_str: &str,
    user_libs: Vec<String>,
    core_wasm_path: String,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
    dry_run: bool,
    is_component: bool,
) {
    run_script(
        &script,
        app_path_str,
        fs::read(app_path_str).unwrap(),
        user_libs.clone(),
        core_wasm_path.clone(),
        Some(instr_app_path.clone()),
        false,
        dry_run,
    );

    // run the instrumented application on wasmtime
    if is_component {
        run_wasmtime_component(
            user_libs,
            core_wasm_path,
            exp_output,
            outdir,
            instr_app_path,
        )
    } else {
        run_wasmtime_module(
            user_libs,
            core_wasm_path,
            exp_output,
            outdir,
            instr_app_path,
        )
    }
}

fn run_testcase_wei(
    script: &Path,
    app_path_str: &str,
    user_libs: Vec<String>,
    core_wasm_path: String,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
    dry_run: bool,
    is_component: bool,
) {
    if is_component {
        todo!("Haven't supported components on wizard yet!")
    }
    let engine_libs = ["whamm:dyninstr"];
    let mut libs_to_link = "".to_string();
    for path in user_libs.iter() {
        let parts = path.split('=').collect::<Vec<&str>>();
        let lib_name_chunk = parts.first().unwrap().to_string();
        let name_parts = lib_name_chunk.split('(').collect::<Vec<&str>>();
        let lib_name = name_parts.first().unwrap().to_string();
        if engine_libs.contains(&&*lib_name) {
            continue;
        }
        if name_parts.len() > 1
            && engine_libs.contains(
                &&*name_parts
                    .get(1)
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .to_string(),
            )
        {
            continue;
        }
        assert_eq!(2, parts.len(), "A user lib should be specified using the following format: <lib_name>=/path/to/lib.wasm");
        libs_to_link += &format!("+{}", parts.get(1).unwrap());
    }

    // run the script on configured application
    if let Err(errs) = run_script(
        script,
        app_path_str,
        vec![],
        user_libs,
        core_wasm_path.clone(),
        Some(instr_app_path.clone()),
        true,
        dry_run,
    ) {
        println!("failed to run script due to errors: ");
        for e in errs.iter() {
            println!("- {}", e.msg)
        }
        panic!()
    }

    // run the instrumented application on wizard
    let wizeng_path = "output/tests/engines/wizeng";

    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());
    let mut cmd = Command::new(wizeng_path);
    // if matches!(exp_output, ExpectedOutput::Hash(_)) {
    cmd.stdout(File::create(out_file.clone()).expect("failed to open log"));
    // }

    // TODO -- uncomment once we figure out the OOM issue:
    //         https://github.com/ejrgilbert/whamm/actions/runs/16132265689/job/45521736032?pr=237
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // {
    //     cmd.arg("--mode=jit");
    // }

    let res = cmd
        // .arg("-tw")
        .arg("--env=TO_CONSOLE=true")
        .arg(format!("--monitors={}+{}{}", instr_app_path, DEFAULT_CORE_LIB_PATH_MODULE, libs_to_link))
        .arg(app_path_str)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run wizard command, please make sure the wizeng executable is available at the path: {}", wizeng_path));
    if !res.status.success() {
        println!(
            "[ERROR] Failed to run wei monitor:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
        panic!();
    } else {
        match exp_output {
            ExpectedOutput::Str(exp_str) => {
                let stdout = fs::read_to_string(&out_file)
                    .unwrap_or_else(|_| panic!("Unable to read file at {:?}", out_file));
                assert_eq!(stdout.trim(), exp_str.trim());
            }
            ExpectedOutput::Hash(exp_hash) => {
                let hash = file_hash(&PathBuf::from(out_file));
                assert_eq!(hash, exp_hash);
            }
        };
    }
}

fn run_wasmtime_component(
    user_libs: Vec<String>,
    core_wasm_path: String,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
) {
    let composed_app_path = wac(instr_app_path, outdir, "whamm-core", &core_wasm_path);

    let mut cmd = Command::new("wasmtime");
    let out_file = prep_outfile(&mut cmd, outdir, &exp_output);
    cmd.arg("run").arg("--env").arg("TO_CONSOLE=true");

    if !user_libs.is_empty() {
        todo!("Haven't supported user libraries for components yet!")
    }

    cmd.arg(composed_app_path);
    run_and_assert(&mut cmd, instr_app_path, &out_file, exp_output);
}

fn run_wasmtime_module(
    user_libs: Vec<String>,
    core_wasm_path: String,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
) {
    let whamm_core_lib_path = format!("whamm_core={core_wasm_path}");
    let mut cmd = Command::new("wasmtime");
    let out_file = prep_outfile(&mut cmd, outdir, &exp_output);
    cmd.arg("run").arg("--env").arg("TO_CONSOLE=true");

    for lib in user_libs.iter() {
        cmd.arg("--preload").arg(format!("{lib}"));
    }

    cmd.arg("--preload")
        .arg(whamm_core_lib_path)
        .arg(instr_app_path);

    run_and_assert(&mut cmd, instr_app_path, &out_file, exp_output);
}

fn prep_outfile(cmd: &mut Command, outdir: &String, exp_output: &ExpectedOutput) -> String {
    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());
    // if matches!(exp_output, ExpectedOutput::Hash(_)) {
    cmd.stdout(File::create(out_file.clone()).expect("failed to open log"));
    // }

    out_file
}

fn run_and_assert(
    cmd: &mut Command,
    app_path: &String,
    out_file: &String,
    exp_output: ExpectedOutput,
) {
    let res = cmd.output().expect("failed to run on engine!");
    if !res.status.success() {
        println!(
            "[ERROR] Failed to run on engine @{app_path}:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
        assert!(false);
    } else {
        assert!(
            res.stderr.is_empty(),
            "Had error: {}",
            String::from_utf8(res.stderr).unwrap()
        );
        match exp_output {
            ExpectedOutput::Str(exp_str) => {
                let stdout = fs::read_to_string(&out_file)
                    .unwrap_or_else(|_| panic!("Unable to read file at {:?}", out_file));
                assert_eq!(stdout.trim(), exp_str.trim());
            }
            ExpectedOutput::Hash(exp_hash) => {
                let hash = file_hash(&PathBuf::from(out_file));
                assert_eq!(hash, exp_hash);
            }
        };
    }
}

fn wac(app_path: &String, outdir: &String, core_lib_name: &str, core_lib_path: &str) -> String {
    let mut graph = CompositionGraph::new();

    // Register the package dependencies into the graph
    let package = Package::from_file("app", None, app_path, graph.types_mut()).unwrap();
    let app = graph.register_package(package).unwrap();

    let package =
        Package::from_file(core_lib_name, None, core_lib_path, graph.types_mut()).unwrap();
    let whamm_core = graph.register_package(package).unwrap();

    // print out some helpful information about what the imports/exports are from the packages.
    println!("LIB EXPORTS:");
    for (name, ty) in &graph.types()[graph[whamm_core].ty()].exports {
        println!("- {name}: {:?}", ty);
    }
    println!("APP IMPORTS");
    for (name, ty) in &graph.types()[graph[app].ty()].imports {
        println!("- {name}: {:?}", ty);
    }
    println!("APP EXPORTS");
    let mut run_func_name = None;
    for (name, ty) in &graph.types()[graph[app].ty()].exports {
        if name.starts_with(RUN_FUNC_PREFIX) {
            run_func_name = Some(name.clone());
        }
        println!("- {name}: {:?}", ty);
    }

    // Instantiate the whamm_core instance which does not have any arguments
    let whamm_core_instance = graph.instantiate(whamm_core);

    // Instantiate the app instance which has a single argument "whamm-core"
    // which is an instance of `whamm_core`
    let app_instance = graph.instantiate(app);

    // plug in the instance of `whamm_core` into the `app` import.
    graph
        .set_instantiation_argument(app_instance, core_lib_name, whamm_core_instance)
        .unwrap();

    // Export the "run" function from the app
    if let Some(run_name) = run_func_name {
        let run_export = graph
            .alias_instance_export(app_instance, &run_name)
            .unwrap();
        graph.export(run_export, &run_name).unwrap();
    } else {
        panic!("Could not find an exported main function from the component, should start with: {RUN_FUNC_PREFIX}")
    }

    // Encode the graph into a WASM binary
    let encoding = graph.encode(EncodeOptions::default()).unwrap();
    let composed_path = format!("{outdir}/composition.wasm");
    fs::write(&composed_path, encoding).unwrap();

    composed_path
}

struct TestCase {
    script: PathBuf,
    app: PathBuf,
    libs: PathBuf,
    exp: PathBuf,
}
