mod common;

use crate::common::{run_basic_instrumentation, run_whamm_bin, wat2wasm_on_dir};
use log::error;
use orca_wasm::Module;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use whamm::common::error::ErrorGen;
use whamm::common::instr::{parse_user_lib_paths, Config, LibraryLinkStrategy};
use whamm::wast::test_harness::wasm2wat_on_file;

const APP_WASM_PATH: &str = "tests/apps/core_suite/handwritten/basic.wasm";
const CORE_WASM_PATH: &str = "./whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";

#[test]
fn run_wast_tests() {
    common::setup_logger();
    whamm::wast::test_harness::run_all().expect("WAST Tests failed!");
}

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// scripts without errors occurring.
#[test]
fn instrument_dfinity_with_fault_injection() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_fault_injection("dfinity");
    assert!(!processed_scripts.is_empty());
    err.fatal_report("Integration Test");
    wat2wasm_on_dir("tests/apps/core_suite/handwritten");

    let wasm_path = "tests/apps/dfinity/users.wasm";
    let wasm = fs::read(wasm_path).unwrap();

    for (script_path, script_text) in processed_scripts {
        let mut module_to_instrument = Module::parse(&wasm, false).unwrap();
        run_script(
            &script_text,
            &script_path,
            &mut module_to_instrument,
            vec![],
            None,
            false,
            &mut err,
        );
    }
}

#[test]
fn instrument_handwritten_wasm_call() {
    common::setup_logger();
    let original_wat_path = "tests/apps/core_suite/handwritten/add.wat";
    let original_wasm_path = "tests/apps/core_suite/handwritten/add.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/tests/integration-handwritten_add.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
}

#[test]
fn instrument_no_matches() {
    common::setup_logger();
    let original_wat_path = "tests/apps/core_suite/handwritten/no_matched_events.wat";
    let original_wasm_path = "tests/apps/core_suite/handwritten/no_matched_events.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/tests/integration-no_matched_events.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
}

#[test]
fn instrument_control_flow() {
    common::setup_logger();
    // Add the target
    let res = Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("wasm32-wasip1")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'instrument_control_flow' add target failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }

    // Build the control_flow Rust project
    let res = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasip1")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'instrument_control_flow' build project failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
    assert!(res.status.success());

    let monitor_path = "tests/scripts/instr.mm";
    let original_wasm_path = "wasm_playground/control_flow/target/wasm32-wasip1/debug/cf.wasm";
    let instrumented_wasm_path = "output/tests/integration-control_flow.wasm";

    run_whamm_bin(original_wasm_path, monitor_path, instrumented_wasm_path);
    wasm2wat_on_file(instrumented_wasm_path);
}

#[test]
fn instrument_spin_with_fault_injection() {
    common::setup_logger();
    let processed_scripts = common::setup_fault_injection("spin");
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_wizard_monitors() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_wizard_monitors();
    assert!(!processed_scripts.is_empty());
    err.fatal_report("Integration Test");

    build_whamm_core_lib();
    wat2wasm_on_dir("tests/apps/core_suite/handwritten");
    let wasm = fs::read(APP_WASM_PATH).unwrap();
    for (script_path, script_text) in processed_scripts {
        let mut module_to_instrument = Module::parse(&wasm, false).unwrap();
        run_script(
            &script_text,
            &script_path,
            &mut module_to_instrument,
            vec![],
            None,
            false,
            &mut err,
        );
    }
}

#[test]
fn instrument_with_replay() {
    common::setup_logger();
    let processed_scripts = common::setup_replay();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_numerics_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_numerics_monitors();
    assert!(!processed_scripts.is_empty());

    run_core_suite("numerics", processed_scripts, true, true)
}

#[test]
fn instrument_with_paper_eval_branches_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("paper_eval/branches");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-branches", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_categories_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("paper_eval/categories");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-categories", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_hotness_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("paper_eval/hotness");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-hotness", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_ins_count_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("paper_eval/ins_count");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-ins_count", processed_scripts, true, true)
}

#[test]
fn instrument_with_paper_eval_cache_sim_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("paper_eval/cache_sim");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-cache_sim", processed_scripts, true, true)
}

#[test]
fn instrument_with_branch_monitor_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("core_suite/branch-monitor");
    assert!(!processed_scripts.is_empty());

    // TODO -- fix wizard side (THEN merge with below test)
    //   - pull `fname`, `targets`, `num_targets`, `default_target`
    run_core_suite("branch-monitor", processed_scripts, true, true)
}
#[test]
fn instrument_with_branch_monitor_rewriting_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("core_suite/branch-monitor_rewriting");
    assert!(!processed_scripts.is_empty());

    run_core_suite("branch-monitor_rewriting", processed_scripts, true, false)
}

#[test]
fn instrument_with_calls_monitor_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("core_suite/calls-monitor");
    assert!(!processed_scripts.is_empty());

    // TODO -- fix wizard side (THEN merge with below test)
    //   - pull `fname`
    run_core_suite("calls-monitor", processed_scripts, true, true)
}
#[test]
fn instrument_with_calls_monitor_rewriting_scripts() {
    common::setup_logger();
    let processed_scripts = common::setup_tests("core_suite/calls-monitor_rewriting");
    assert!(!processed_scripts.is_empty());

    run_core_suite("calls-monitor_rewriting", processed_scripts, true, false)
}

struct TestCase {
    script: PathBuf,
    script_str: String,
    app: PathBuf,
    libs: PathBuf,
    exp: PathBuf,
}

fn run_core_suite(
    suite_name: &str,
    processed_scripts: Vec<(PathBuf, String)>,
    with_br: bool,
    with_wizard: bool,
) {
    build_whamm_core_lib();
    build_user_libs();
    // wat2wasm_on_dir("tests/apps/core_suite/rust");
    // wat2wasm_on_dir("tests/apps/core_suite/handwritten");
    // wat2wasm_on_dir("tests/apps/core_suite/clang");

    let mut rewriting_tests = vec![];
    let mut wizard_tests = vec![];
    for (script_path, script_str) in processed_scripts.iter() {
        let fname = script_path.file_name().unwrap().to_str().unwrap();
        let path = script_path.parent().unwrap();

        let app = path.join("app").join(format!("{}.app", fname));
        let libs = path.join("libs").join(format!("{}.libs", fname));
        let rewriting_exp = path
            .join("expected")
            .join("rewriting")
            .join(format!("{}.exp", fname));
        let wizard_exp = path
            .join("expected")
            .join("wizard")
            .join(format!("{}.exp", fname));

        rewriting_tests.push(TestCase {
            script: script_path.clone(),
            script_str: script_str.clone(),
            app: app.clone(),
            libs: libs.clone(),
            exp: rewriting_exp,
        });
        wizard_tests.push(TestCase {
            script: script_path.clone(),
            script_str: script_str.clone(),
            app,
            libs,
            exp: wizard_exp,
        });
    }

    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    err.fatal_report("Integration Test");
    let outdir = format!("output/tests/{suite_name}");
    try_path(&outdir);
    let instr_app_path = format!("{outdir}/output.wasm");

    if with_br {
        for TestCase {
            script,
            script_str,
            app,
            libs,
            exp,
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
                Some(libs)
            } else {
                None
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
                script_str,
                &app_path_str,
                libs_path_str,
                exp_out,
                &outdir,
                &instr_app_path,
                &mut err,
            );
        }
    }

    if with_wizard {
        for TestCase {
            script,
            script_str,
            app,
            libs,
            exp,
        } in wizard_tests.iter()
        {
            println!(
                "[WIZARD] Running test case with monitor at the following path: {:#?}",
                script
            );
            let app_path_str = fs::read_to_string(app)
                .unwrap_or_else(|_| panic!("Unable to read file at {:?}", app));
            let libs_path_str = if let Ok(res) = fs::read_to_string(libs) {
                let mut libs = vec![];
                for lib in res.split('\n') {
                    libs.push(lib.to_string());
                }
                Some(libs)
            } else {
                None
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
            run_testcase_wizard(
                script,
                script_str,
                &app_path_str,
                libs_path_str,
                exp_out,
                &outdir,
                &instr_app_path,
                &mut err,
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

fn build_lib(lib_path: &str) {
    let res = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasip1")
        .arg("--release")
        .current_dir(lib_path)
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

fn build_whamm_core_lib() {
    // Build the whamm_core library
    build_lib("whamm_core");
}

fn build_user_libs() {
    let lib_projects = fs::read_dir("./user_libs").unwrap();

    for path in lib_projects {
        build_lib(path.unwrap().path().display().to_string().as_str());
    }
}

/// create output path if it doesn't exist
pub(crate) fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

fn run_script(
    script_text: &String,
    script_path: &PathBuf,
    target_wasm: &mut Module,
    user_libs: Vec<(String, String, Vec<u8>)>,
    output_path: Option<String>,
    target_wizard: bool,
    err: &mut ErrorGen,
) {
    let script_path_str = script_path.to_str().unwrap().replace("\"", "");
    let wasm_result = whamm::common::instr::run(
        CORE_WASM_PATH,
        "./",
        target_wasm,
        &script_text,
        &script_path_str,
        user_libs,
        0,
        Config {
            wizard: target_wizard,
            enable_wizard_alt: false,
            metrics: false,
            no_bundle: false,
            no_body: false,
            no_pred: false,
            no_report: false,
            testing: true,
            library_strategy: LibraryLinkStrategy::Imported,
        },
    );
    if let Some(path) = output_path {
        try_path(&path);
        if let Err(e) = fs::write(&path, wasm_result) {
            unreachable!(
                "Failed to dump instrumented wasm to {} from error: {}",
                &path, e
            )
        }
    }
    err.fatal_report("Integration Test");
}

fn run_testcase_rewriting(
    script: &PathBuf,
    script_str: &String,
    app_path_str: &str,
    user_libs_arg: Option<Vec<String>>,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
    err: &mut ErrorGen,
) {
    let user_libs = if let Some(user_lib_paths) = &user_libs_arg {
        parse_user_lib_paths(user_lib_paths.clone())
    } else {
        vec![]
    };

    // run the script on configured application
    let wasm = fs::read(app_path_str).unwrap();
    let mut module_to_instrument = Module::parse(&wasm, false).unwrap();
    run_script(
        &script_str,
        &script,
        &mut module_to_instrument,
        user_libs,
        Some(instr_app_path.clone()),
        false,
        err,
    );

    // let home = match env::var("HOME") {
    //     Ok(val) => val,
    //     Err(_) => panic!("Could not find HOME environment variable"),
    // };

    // run the instrumented application on wasmtime
    // let res = Command::new(format!("{home}/.cargo/bin/cargo"))

    let whamm_core_lib_path = "whamm_core=whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";
    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());
    let mut cmd = Command::new("wasmtime");
    if matches!(exp_output, ExpectedOutput::Hash(_)) {
        cmd.stdout(File::create(out_file.clone()).expect("failed to open log"));
    }
    cmd.arg("run").arg("--env").arg("TO_CONSOLE=true");

    if let Some(libs) = &user_libs_arg {
        for lib in libs.iter() {
            cmd.arg("--preload").arg(format!("{lib}"));
        }
    }

    let res = cmd
        .arg("--preload")
        .arg(whamm_core_lib_path)
        .arg(instr_app_path)
        .output()
        .expect("failed to run on wasmtime");
    if !res.status.success() {
        println!(
            "[ERROR] Failed to run on wasmtime:\n{}\n{}",
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
                let stdout = String::from_utf8(res.stdout).unwrap();
                assert_eq!(stdout.trim(), exp_str.trim());
            }
            ExpectedOutput::Hash(exp_hash) => {
                let hash = file_hash(&PathBuf::from(out_file));
                assert_eq!(hash, exp_hash);
            }
        };
    }
}

fn run_testcase_wizard(
    script: &PathBuf,
    script_str: &String,
    app_path_str: &str,
    user_libs: Option<Vec<String>>,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
    err: &mut ErrorGen,
) {
    let user_libs = if let Some(user_lib_paths) = user_libs {
        parse_user_lib_paths(user_lib_paths)
    } else {
        vec![]
    };

    let mut libs_to_link = "".to_string();
    for (_, lib_path, _) in user_libs.iter() {
        libs_to_link += &format!("+{lib_path}");
    }

    // run the script on configured application
    let mut module_to_instrument = Module::default();
    run_script(
        &script_str,
        &script,
        &mut module_to_instrument,
        user_libs,
        Some(instr_app_path.clone()),
        true,
        err,
    );

    // run the instrumented application on wizard
    let whamm_core_lib_path = "whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";
    let wizeng_path = "output/tests/engines/wizeng";

    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());
    let mut cmd = Command::new(wizeng_path);
    if matches!(exp_output, ExpectedOutput::Hash(_)) {
        cmd.stdout(File::create(out_file.clone()).expect("failed to open log"));
    }

    let res = cmd.arg("--env=TO_CONSOLE=true")
        .arg(format!("--monitors={}+{}{}", instr_app_path, whamm_core_lib_path, libs_to_link))
        .arg(app_path_str)
        .output()
        .expect(&format!("Failed to run wizard command, please make sure the wizeng executable is available at the path: {}", wizeng_path));
    if !res.status.success() {
        println!(
            "[ERROR] Failed to run wizard monitor:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
        assert!(false);
    } else {
        match exp_output {
            ExpectedOutput::Str(exp_str) => {
                let stdout = String::from_utf8(res.stdout).unwrap();
                assert_eq!(stdout.trim(), exp_str.trim());
            }
            ExpectedOutput::Hash(exp_hash) => {
                let hash = file_hash(&PathBuf::from(out_file));
                assert_eq!(hash, exp_hash);
            }
        };
    }
}
