use crate::util::{setup_logger, CORE_WASM_PATH};
use anyhow::{anyhow, Context, Result};
use glob::{glob, glob_with};
use log::{error, warn};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use whamm::api::instrument::{instrument_as_dry_run_rewriting, Diagnostics, WhammError};
use whamm::api::utils::{validate_wasm_file, write_to_file};
use wirm::Module;

const TEST_DRY_RUN: bool = true;
pub const DEFAULT_CORE_LIB_PATH: &str = "tests/libs/whamm_core.wasm";
pub const DEFAULT_DEFS_PATH: &str = "./";
const TEST_RSC_DIR: &str = "tests/scripts/";
const MM_PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

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
            .unwrap_or_else(|_| panic!("Unable to read file at {path:?}"));
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
    assert!(
        res.status.success(),
        "'run_whamm_bin' add target failed:\n{}\n{}",
        String::from_utf8(res.stdout).unwrap(),
        String::from_utf8(res.stderr).unwrap()
    );
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
        DEFAULT_CORE_LIB_PATH,
    );
    validate_wasm_file(instrumented_wasm_path);
}

pub fn wat2wasm_on_file(original_wat_path: &str, original_wasm_path: &str) {
    match wat::parse_file(original_wat_path) {
        Ok(wasm) => fs::write(original_wasm_path, wasm)
            .unwrap_or_else(|e| panic!("failed to write {original_wasm_path}: {e}")),
        Err(e) => error!("'wat::parse_file' failed on {original_wat_path}: {e}"),
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
    with_br: bool,
    with_wei: bool,
) -> Result<()> {
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
            let expected_err = read_expected_err(exp);
            if let Some(ref expected_err) = expected_err {
                if !exp.exists() {
                    run_testcase_expect_error(
                        script,
                        &app_path_str,
                        libs_path_str,
                        expected_err,
                        false,
                    );
                    continue;
                }
            }
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
                exp_out,
                &outdir,
                &instr_app_path,
                expected_err.as_deref(),
            )?;
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
            let expected_err = read_expected_err(exp);
            if let Some(ref expected_err) = expected_err {
                if !exp.exists() {
                    run_testcase_expect_error(
                        script,
                        &app_path_str,
                        libs_path_str,
                        expected_err,
                        true,
                    );
                    continue;
                }
            }
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
                exp_out,
                &outdir,
                &instr_app_path,
                expected_err.as_deref(),
            )?;
        }
    }
    Ok(())
}
const MAX_EXP_OUT_SIZE: u64 = 50_000; // 50 KB
pub(crate) enum ExpectedOutput {
    Hash(String),
    Str(String),
    /// Skip the output comparison entirely — only assert that compile + run succeed.
    None,
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
    target_wasm: &mut Module,
    user_libs: Vec<String>,
    output_path: Option<String>,
    target_wei: bool,
) -> Result<Diagnostics> {
    let script_path_str = script_path
        .to_str()
        .ok_or_else(|| anyhow!("script path is not valid UTF-8: {script_path:?}"))?
        .replace("\"", "");
    let script_bytes = std::fs::read(&script_path_str)
        .with_context(|| format!("could not read script {script_path_str}"))?;
    let script_text = String::from_utf8_lossy(&script_bytes).into_owned();
    let user_lib_bytes = whamm::api::parse_user_libs(user_libs.clone())?;
    let core_lib_bytes = Some(whamm::api::load_core_lib_from_path(Path::new(
        CORE_WASM_PATH,
    ))?);
    let defs_bytes = Some(whamm::api::load_defs_from_path(Path::new("./")));
    let instrumented = if target_wei {
        whamm::api::instrument::generate_monitor_module(
            script_bytes.clone(),
            user_lib_bytes.clone(),
            core_lib_bytes.clone(),
            defs_bytes.clone(),
        )
    } else {
        whamm::api::instrument::instrument_module_with_rewriting(
            target_wasm,
            script_bytes.clone(),
            user_lib_bytes.clone(),
            core_lib_bytes.clone(),
            defs_bytes.clone(),
        )
    }
    .map_err(|diags| format_whamm_diags(&script_text, &diags))?;
    let wasm_result = instrumented.module;
    let diagnostics = instrumented.diagnostics;
    if TEST_DRY_RUN && !target_wei {
        let wasm_bytes = std::fs::read(wasm_path)
            .with_context(|| format!("could not read app wasm {wasm_path}"))?;
        let _side_effects = instrument_as_dry_run_rewriting(
            wasm_bytes,
            script_bytes,
            user_lib_bytes,
            core_lib_bytes,
            defs_bytes,
        )
        .map_err(|errs| format_whamm_errs(&script_text, errs))
        .context("failed to run dry-run")?;

        // NOTE: uncomment to debug side effects...just don't commit this uncommented! it'll slow EVERYTHING down
        // print_side_effects(&_side_effects);
    }
    if let Some(path) = output_path {
        write_to_file(wasm_result, Path::new(&path));
    }
    Ok(diagnostics)
}

fn format_whamm_diags(script: &str, diags: &Diagnostics) -> anyhow::Error {
    let body = diags
        .errors()
        .map(|d| format_entry(script, &d.msg, d.err_line_col()))
        .collect::<Vec<_>>()
        .join("\n");
    anyhow!("{} error(s) from whamm:\n{body}", diags.errors().count())
}

fn format_whamm_errs(script: &str, errs: Vec<WhammError>) -> anyhow::Error {
    let body = errs
        .iter()
        .map(|e| format_entry(script, &e.msg, e.err_line_col()))
        .collect::<Vec<_>>()
        .join("\n");
    anyhow!("{} error(s) from whamm:\n{body}", errs.len())
}

fn format_entry(script: &str, msg: &str, line_col: Option<(usize, usize)>) -> String {
    let mut entry = format!("  - {msg}");
    if let Some((line, _col)) = line_col {
        if let Some(src) = script.lines().nth(line - 1) {
            entry.push_str(&format!("\n    --> `{}` (line {line})", src.trim()));
        }
    }
    entry
}

/// If a `.err` file sits beside the given `.exp` path, return its contents (the
/// expected diagnostic). Its presence marks the script as expected-to-fail.
fn read_expected_err(exp: &Path) -> Option<String> {
    fs::read_to_string(exp.with_extension("err")).ok()
}

/// Assert that instrumenting `script` emits a diagnostic containing `expected_err`
fn run_testcase_expect_error(
    script: &Path,
    app_path_str: &str,
    user_libs: Vec<String>,
    expected_err: &str,
    target_wei: bool,
) {
    let label = if target_wei { "WEI" } else { "REWRITE" };
    let wasm = fs::read(app_path_str)
        .unwrap_or_else(|_| panic!("Unable to read app wasm at {app_path_str}"));
    let mut module = if target_wei {
        Module::default()
    } else {
        Module::parse(&wasm, false, true).unwrap()
    };
    match run_script(
        script,
        app_path_str,
        &mut module,
        user_libs.clone(),
        None,
        target_wei,
    ) {
        Ok(diagnostics) => {
            let actual = diagnostics.render(false);
            let expected = expected_err.trim();
            assert!(
                actual.contains(expected),
                "[{label}] {script:?} succeeded without the expected diagnostic\n\
                 --- expected (substring) ---\n{expected}\n--- actual (warnings) ---\n{actual}"
            );
        }
        Err(e) => {
            let actual = format!("{e:#}");
            let expected = expected_err.trim();
            assert!(
                actual.contains(expected),
                "[{label}] error mismatch for {script:?}\n\
                 --- expected (substring) ---\n{expected}\n--- actual ---\n{actual}"
            );
        }
    }
}

/// Output of instrumented module run
pub(crate) struct WasmtimeRun {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// The trap/exit message
    pub error: Option<String>,
}

/// Split a `name=path` preload spec, as passed to `wasmtime run --preload`.
fn split_preload(lib: &str) -> Result<(String, String)> {
    let (name, path) = lib
        .split_once('=')
        .ok_or_else(|| anyhow!("preload lib must be `name=path`, got: {lib}"))?;
    Ok((name.to_string(), path.to_string()))
}

pub(crate) fn run_on_wasmtime(
    app_path: &str,
    preloads: &[(String, String)],
) -> Result<WasmtimeRun> {
    use wasmtime::{Config, Engine, Linker, Module as WtModule, Store};
    use wasmtime_wasi::p1::{add_to_linker_sync, WasiP1Ctx};
    use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
    use wasmtime_wasi::{I32Exit, WasiCtxBuilder};

    // Enable the same proposals the `wasmtime run --wasm <feature>=y` flags did.
    let mut config = Config::new();
    config
        .wasm_threads(true)
        .shared_memory(true)
        .wasm_custom_page_sizes(true)
        .wasm_function_references(true)
        .wasm_gc(true)
        .wasm_exceptions(true);
    let engine = Engine::new(&config)?;

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    add_to_linker_sync(&mut linker, |s| s)?;

    // Generous cap; the buffer only grows as the guest writes.
    const PIPE_CAPACITY: usize = 512 << 20;
    let stdout = MemoryOutputPipe::new(PIPE_CAPACITY);
    let stderr = MemoryOutputPipe::new(PIPE_CAPACITY);
    let wasi = WasiCtxBuilder::new()
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .env("TO_CONSOLE", "true")
        .build_p1();
    let mut store = Store::new(&engine, wasi);

    for (name, path) in preloads {
        let module = WtModule::from_file(&engine, path)
            .map_err(|e| anyhow!("failed to load preload module `{name}` from {path}: {e}"))?;
        linker.module(&mut store, name, &module)?;
    }
    let app = WtModule::from_file(&engine, app_path)
        .map_err(|e| anyhow!("failed to load app module from {app_path}: {e}"))?;
    linker.module(&mut store, "", &app)?;

    let result = linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ());

    let (success, error) = match result {
        Ok(()) => (true, None),
        Err(e) => match e.downcast_ref::<I32Exit>() {
            Some(I32Exit(0)) => (true, None),
            Some(I32Exit(code)) => (false, Some(format!("exited with code {code}"))),
            None => (false, Some(format!("{e:?}"))),
        },
    };
    Ok(WasmtimeRun {
        success,
        stdout: stdout.contents().to_vec(),
        stderr: stderr.contents().to_vec(),
        error,
    })
}

pub(crate) fn run_testcase_rewriting(
    script: &Path,
    app_path_str: &str,
    user_libs: Vec<String>,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &str,
    expected_err: Option<&str>,
) -> Result<()> {
    // run the script on configured application
    let wasm = fs::read(app_path_str)?;
    let mut module_to_instrument = Module::parse(&wasm, true, true)?;
    let diagnostics = run_script(
        script,
        app_path_str,
        &mut module_to_instrument,
        user_libs.clone(),
        Some(instr_app_path.to_owned()),
        false,
    )?;
    match expected_err {
        // make sure it errored in the specified way (.err)
        Some(expected_err) => assert!(
            diagnostics.render(false).contains(expected_err.trim()),
            "[REWRITE] {script:?} did not emit the expected diagnostic\n\
             --- expected (substring) ---\n{}\n--- actual ---\n{}",
            expected_err.trim(),
            diagnostics.render(false)
        ),
        // make sure it instrumented cleanly
        None => assert!(
            diagnostics.is_empty(),
            "[REWRITE] {script:?} produced unexpected whamm diagnostics \
             (ship a `.err` file if the diagnostic is intended):\n{}",
            diagnostics.render(false)
        ),
    }

    // run the instrumented application on wasmtime
    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());

    let mut preloads = user_libs
        .iter()
        .map(|lib| split_preload(lib))
        .collect::<Result<Vec<_>>>()?;
    preloads.push((
        "whamm_core".to_string(),
        "tests/libs/whamm_core.wasm".to_string(),
    ));

    let run = run_on_wasmtime(instr_app_path, &preloads)?;
    if !run.success {
        println!(
            "[ERROR] Failed to run on wasmtime:\n{}\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr),
            run.error.unwrap_or_default()
        );
        panic!();
    }
    assert!(
        run.stderr.is_empty(),
        "Had error: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    match exp_output {
        ExpectedOutput::Str(exp_str) => {
            let stdout = String::from_utf8(run.stdout).unwrap();
            fs::write(out_file, &stdout).expect("failed to write to log");
            assert_eq!(stdout.trim(), exp_str.trim());
        }
        ExpectedOutput::Hash(exp_hash) => {
            fs::write(&out_file, &run.stdout).expect("failed to write to log");
            let hash = file_hash(&PathBuf::from(out_file));
            assert_eq!(hash, exp_hash);
        }
        ExpectedOutput::None => {}
    };
    Ok(())
}

pub(crate) fn run_testcase_wei(
    script: &Path,
    app_path_str: &str,
    user_libs: Vec<String>,
    exp_output: ExpectedOutput,
    outdir: &String,
    instr_app_path: &String,
    expected_err: Option<&str>,
) -> Result<()> {
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
    let mut module_to_instrument = Module::default();
    let diagnostics = run_script(
        script,
        app_path_str,
        &mut module_to_instrument,
        user_libs,
        Some(instr_app_path.clone()),
        true,
    )?;
    // make sure it errored in the specified way (.err)
    if let Some(expected_err) = expected_err {
        assert!(
            diagnostics.render(false).contains(expected_err.trim()),
            "[WEI] {script:?} did not emit the expected diagnostic\n\
             --- expected (substring) ---\n{}\n--- actual ---\n{}",
            expected_err.trim(),
            diagnostics.render(false)
        );
    }

    // run the instrumented application on wizard
    let whamm_core_lib_path = "tests/libs/whamm_core.wasm";
    let wizeng_path = "output/tests/engines/wizeng";

    let out_filename = "instr-flush.out";
    let out_file = format!("{outdir}/{out_filename}");
    let _ = fs::remove_file(out_file.clone());
    let mut cmd = Command::new(wizeng_path);
    if matches!(exp_output, ExpectedOutput::Hash(_)) {
        cmd.stdout(File::create(out_file.clone()).expect("failed to open log"));
    }

    // TODO -- uncomment once we figure out the OOM issue:
    //         https://github.com/ejrgilbert/whamm/actions/runs/16132265689/job/45521736032?pr=237
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // {
    //     cmd.arg("--mode=jit");
    // }

    let res = cmd
        // .arg("-tw")
        .arg("--env=TO_CONSOLE=true")
        .arg(format!("--monitors={}+{}{}", instr_app_path, whamm_core_lib_path, libs_to_link))
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
                let stdout = String::from_utf8(res.stdout).unwrap();
                assert_eq!(stdout.trim(), exp_str.trim());
            }
            ExpectedOutput::Hash(exp_hash) => {
                let hash = file_hash(&PathBuf::from(out_file));
                assert_eq!(hash, exp_hash);
            }
            ExpectedOutput::None => {}
        };
    }
    Ok(())
}

struct TestCase {
    script: PathBuf,
    app: PathBuf,
    libs: PathBuf,
    exp: PathBuf,
}
