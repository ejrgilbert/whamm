use crate::util::{setup_logger, CORE_WASM_PATH};
use anyhow::{anyhow, Context, Result};
use glob::{glob, glob_with};
use log::{error, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use whamm::api::instrument::{instrument_as_dry_run_rewriting, Diagnostics, UserLibs, WhammError};
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

pub fn run_whamm(
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
    defs_path: &str,
    core_lib_path: &str,
) {
    let raw = fs::read(original_wasm_path)
        .unwrap_or_else(|e| panic!("failed to read app at {original_wasm_path}: {e}"));
    let wasm_app = wat::parse_bytes(&raw)
        .unwrap_or_else(|e| panic!("failed to parse app at {original_wasm_path}: {e}"))
        .into_owned();
    let script = fs::read(monitor_path)
        .unwrap_or_else(|e| panic!("failed to read script at {monitor_path}: {e}"));
    let core_lib = whamm::api::load_core_lib_from_path(Path::new(core_lib_path))
        .unwrap_or_else(|e| panic!("failed to load core lib at {core_lib_path}: {e}"));
    let defs = whamm::api::load_defs_from_path(Path::new(defs_path));

    match whamm::api::instrument::instrument_with_rewriting(
        wasm_app,
        script,
        UserLibs::default(),
        Some(core_lib),
        Some(defs),
    ) {
        Ok(instrumented) => {
            write_to_file(instrumented.module, Path::new(instrumented_wasm_path));
        }
        Err(diagnostics) => {
            panic!(
                "'run_whamm_bin' add target failed:\n{}",
                diagnostics.render(false)
            );
        }
    }
}

pub fn run_basic_instrumentation(
    original_wat_path: &str,
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
) {
    wat2wasm_on_file(original_wat_path, original_wasm_path);
    run_whamm(
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
        for tc in rewriting_tests.iter() {
            println!(
                "[REWRITE] Running test case with monitor at the following path: {:#?}",
                tc.script
            );
            let (app_path_str, user_libs, expected_err, exp_out) = load_testcase(tc);
            run_testcase_rewriting(
                &tc.script,
                &app_path_str,
                user_libs,
                exp_out,
                &outdir,
                &instr_app_path,
                expected_err.as_deref(),
            )?;
        }
    }

    if with_wei {
        for tc in wei_tests.iter() {
            println!(
                "[WEI] Running test case with monitor at the following path: {:#?}",
                tc.script
            );
            let (app_path_str, user_libs, expected_err, exp_out) = load_testcase(tc);
            run_testcase_wei(
                &tc.script,
                &app_path_str,
                user_libs,
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
    let instance = linker.instantiate(&mut store, &app)?;
    let result = instance
        .get_typed_func::<(), ()>(&mut store, "_start")?
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

fn load_testcase(tc: &TestCase) -> (String, Vec<String>, Option<String>, ExpectedOutput) {
    let app_path_str = fs::read_to_string(&tc.app)
        .unwrap_or_else(|_| panic!("Unable to read file at {:?}", tc.app));
    let user_libs = match fs::read_to_string(&tc.libs) {
        Ok(res) => res.split('\n').map(str::to_string).collect(),
        Err(_) => vec![],
    };
    let expected_err = read_expected_err(&tc.exp);
    let exp_output = if tc.exp.exists() {
        let metadata = fs::metadata(&tc.exp).unwrap_or_else(|_| {
            panic!(
                "Failed to load expected output file metadata at: {:?}",
                tc.exp
            )
        });
        if metadata.len() > MAX_EXP_OUT_SIZE {
            ExpectedOutput::hash(&tc.exp)
        } else {
            ExpectedOutput::Str(
                fs::read_to_string(&tc.exp)
                    .unwrap_or_else(|_| panic!("Unable to read file at {:?}", tc.exp)),
            )
        }
    } else if expected_err.is_some() {
        ExpectedOutput::None
    } else {
        panic!("No `.exp` or `.err` fixture found for {:?}", tc.script);
    };
    (app_path_str, user_libs, expected_err, exp_output)
}

fn compiled_or_expected_error(
    label: &str,
    script: &Path,
    compiled: Result<Diagnostics>,
    expected_err: Option<&str>,
) -> Result<Option<Diagnostics>> {
    match compiled {
        Ok(diagnostics) => Ok(Some(diagnostics)),
        Err(e) => {
            let Some(expected) = expected_err else {
                return Err(e);
            };
            let actual = format!("{e:#}");
            assert!(
                actual.contains(expected.trim()),
                "[{label}] {script:?} compile error did not match the expected `.err`\n\
                 --- expected (substring) ---\n{}\n--- actual ---\n{actual}",
                expected.trim()
            );
            Ok(None)
        }
    }
}

fn assert_expected_output(stdout: &str, exp_output: &ExpectedOutput, out_file: &str) {
    match exp_output {
        ExpectedOutput::Str(exp_str) => {
            fs::write(out_file, stdout).expect("failed to write to log");
            assert_eq!(stdout.trim(), exp_str.trim());
        }
        ExpectedOutput::Hash(exp_hash) => {
            fs::write(out_file, stdout).expect("failed to write to log");
            assert_eq!(&file_hash(&PathBuf::from(out_file)), exp_hash);
        }
        ExpectedOutput::None => {}
    }
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
    // compile the script onto the configured application
    let wasm = fs::read(app_path_str)?;
    let mut module_to_instrument = Module::parse(&wasm, true, true)?;
    let compiled = run_script(
        script,
        app_path_str,
        &mut module_to_instrument,
        user_libs.clone(),
        Some(instr_app_path.to_owned()),
        false,
    );
    let Some(diagnostics) = compiled_or_expected_error("REWRITE", script, compiled, expected_err)?
    else {
        return Ok(()); // an expected compile error was matched; nothing to run
    };

    match expected_err {
        Some(expected_err) => assert!(
            diagnostics.render(false).contains(expected_err.trim()),
            "[REWRITE] {script:?} did not emit the expected diagnostic\n\
             --- expected (substring) ---\n{}\n--- actual ---\n{}",
            expected_err.trim(),
            diagnostics.render(false)
        ),
        None => assert!(
            diagnostics.is_empty(),
            "[REWRITE] {script:?} produced unexpected whamm diagnostics \
             (ship a `.err` file if the diagnostic is intended):\n{}",
            diagnostics.render(false)
        ),
    }

    // run the instrumented application on wasmtime
    let out_file = format!("{outdir}/instr-flush.out");
    let _ = fs::remove_file(&out_file);

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
    assert_expected_output(&String::from_utf8(run.stdout)?, &exp_output, &out_file);
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

    // compile the script to a wei monitor module
    let mut module_to_instrument = Module::default();
    let compiled = run_script(
        script,
        app_path_str,
        &mut module_to_instrument,
        user_libs,
        Some(instr_app_path.clone()),
        true,
    );

    if compiled_or_expected_error("WEI", script, compiled, expected_err)?.is_none() {
        return Ok(()); // an expected compile error was matched; nothing to run
    }

    // run the instrumented application on wizard
    let whamm_core_lib_path = "tests/libs/whamm_core.wasm";
    let wizeng_path = "output/tests/engines/wizeng";
    let out_file = format!("{outdir}/instr-flush.out");
    let _ = fs::remove_file(&out_file);

    // TODO -- pass `--mode=jit` once the OOM issue is resolved:
    //         https://github.com/ejrgilbert/whamm/actions/runs/16132265689/job/45521736032?pr=237
    let res = Command::new(wizeng_path)
        .arg("--env=TO_CONSOLE=true")
        .arg(format!("--monitors={}+{}{}", instr_app_path, whamm_core_lib_path, libs_to_link))
        .arg(app_path_str)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run wizard command, please make sure the wizeng executable is available at the path: {}", wizeng_path));
    let stdout = String::from_utf8_lossy(&res.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&res.stderr).into_owned();

    if let Some(expected_err) = expected_err {
        assert!(
            format!("{stdout}\n{stderr}").contains(expected_err.trim()),
            "[WEI] {script:?} did not print the expected wizard error\n\
             --- expected (substring) ---\n{}\n--- actual stdout ---\n{stdout}\n--- actual stderr ---\n{stderr}",
            expected_err.trim()
        );
    } else if !res.status.success() {
        println!("[ERROR] Failed to run wei monitor:\n{stdout}\n{stderr}");
        panic!();
    }
    assert_expected_output(&stdout, &exp_output, &out_file);
    Ok(())
}

struct TestCase {
    script: PathBuf,
    app: PathBuf,
    libs: PathBuf,
    exp: PathBuf,
}
