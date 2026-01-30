#![allow(dead_code)]

use crate::api::instrument::{Config, LibraryLinkStrategy};
use crate::api::utils::wasm2wat_on_file;
use crate::common::instr::{run, try_path};
use crate::common::metrics::Metrics;
use crate::parser::yml_processor::pull_all_yml_files;
use log::{debug, error};
use std::fs::{File, remove_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use wirm::Module;

const CORE_WASM_PATH: &str = "tests/libs/whamm_core.wasm";
const DEFS_PATH: &str = "./";
const TEST_DEBUG_DIR: &str = "output/tests/debug_me/";
const OUTPUT_DIR: &str = "output/tests/wast_suite";
const OUTPUT_WHAMMED_WAST: &str = "output/tests/wast_suite/should_pass";
const OUTPUT_UNINSTR_WAST: &str = "output/tests/wast_suite/should_fail";

pub fn run_all() -> Result<(), std::io::Error> {
    clean();

    // Find all the wast files to run as tests
    let wast_tests = find_wast_tests();
    setup_and_run_tests(&wast_tests)?;

    Ok(())
}

/// Clear out the previous test directory
pub(crate) fn clean() {
    remove_dir_all(Path::new(OUTPUT_DIR)).ok();
}

pub fn setup_and_run_tests(wast_tests: &Vec<PathBuf>) -> Result<(), std::io::Error> {
    let (all_wast_should_fail, all_wast_should_pass) = setup(wast_tests)?;

    // Now that we've generated the wast files, let's run them on the configured interpreters!
    run_wast_tests(all_wast_should_fail, all_wast_should_pass);
    Ok(())
}

fn setup(wast_tests: &Vec<PathBuf>) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let mut all_wast_should_pass = vec![];
    let mut all_wast_should_fail = vec![];
    for test in wast_tests {
        let f = File::open(test.clone())?;
        let mut reader = BufReader::new(f);

        let test_setup = get_test_setup(&mut reader, test)?;

        // Get the `whamm!` scripts and corresponding test cases for this module
        let test_cases = get_test_cases(reader);

        for test_case in test_cases.iter() {
            test_case.print();
        }

        match generate_should_fail_bin_wast(&test_setup, &test_cases, test) {
            Err(e) => {
                panic!(
                    "Unable to write UN-instrumented wast file due to error: {:?}",
                    e
                );
            }
            Ok(mut files) => {
                all_wast_should_fail.append(&mut files);
            }
        };

        match generate_instrumented_bin_wast(&test_setup, &test_cases, test) {
            Err(e) => {
                panic!(
                    "Unable to write instrumented wast file due to error: {:?}",
                    e
                );
            }
            Ok(mut files) => all_wast_should_pass.append(&mut files),
        };
    }
    Ok((all_wast_should_fail, all_wast_should_pass))
}

fn run_wast_tests(wast_should_fail: Vec<String>, wast_should_pass: Vec<String>) {
    let inters = get_available_interpreters();
    assert!(
        !inters.is_empty(),
        "No supported interpreters are configured, fail!\n\
        To fix, add an executable binary under {INT_PATH} for one of the following interpreter options:\n\
        1. the wizeng interpreter, named '{WIZENG_SPEC_INT}'. https://github.com/titzer/wizard-engine/tree/master\n\
        2. the Wasm reference interpreter, named '{WASM_REF_INT}'. https://github.com/WebAssembly/spec/tree/main/interpreter\n"
    );

    println!("\n>>> Running wast on the following available interpreters:");
    for (i, (inter, _args)) in inters.iter().enumerate() {
        println!("{i}. {inter}");
    }
    println!();

    run_wast_tests_that_should_fail(&inters, wast_should_fail);
    run_wast_tests_that_should_pass(&inters, wast_should_pass);
}

/// Run all the wast files that should FAIL on each of the configured interpreters
fn run_wast_tests_that_should_fail(inters: &[(String, Vec<String>)], wast_files: Vec<String>) {
    debug!("Running wast tests that should fail.");
    for (inter, args) in inters.iter() {
        for wast in wast_files.iter() {
            let res = run_wast_test(inter, args, wast);
            if res.status.success() {
                error!(
                    "The following command should have FAILED (ran un-instrumented): '{inter} {wast}'"
                );
            }
            assert!(!res.status.success());
        }
    }
}

/// Run all the wast files that should PASS on each of the configured interpreters
fn run_wast_tests_that_should_pass(inters: &[(String, Vec<String>)], wast_files: Vec<String>) {
    debug!("Running wast tests that should pass.");
    for (inter, args) in inters.iter() {
        for wast in wast_files.iter() {
            let res = run_wast_test(inter, args, wast);
            if !res.status.success() {
                error!(
                    "The following command should have PASSED: '{inter} {wast}'\n{}\n{}",
                    String::from_utf8(res.stdout).unwrap(),
                    String::from_utf8(res.stderr).unwrap()
                );
            }
            assert!(res.status.success());
        }
    }
}

fn run_wast_test(inter: &String, args: &[String], wast_file_name: &String) -> Output {
    let mut command = &mut Command::new(inter);
    for arg in args.iter() {
        command = command.arg(arg);
    }
    command
        .arg(wast_file_name)
        .output()
        .expect("failed to execute process")
}

const INT_PATH: &str = "./output/tests/engines";
const WIZENG_SPEC_INT: &str = "wizard-spectest";
const WASM_REF_INT: &str = "wasm";
fn get_available_interpreters() -> Vec<(String, Vec<String>)> {
    let supported_interpreters = [
        (WASM_REF_INT, vec![]),
        (WIZENG_SPEC_INT, vec!["-ext:multi-memory".to_string()]),
    ];
    let mut available_interpreters = Vec::new();

    for (interpreter, args) in supported_interpreters.iter() {
        let int_path = format!("{INT_PATH}/{interpreter}");
        match Command::new(&int_path).arg("-help").output() {
            Err(..) => {
                // do nothing
            }
            Ok(res) => {
                if res.status.success() {
                    available_interpreters.push((int_path, args.clone()));
                }
            }
        }
    }

    available_interpreters
}

// ==============================
// ---- WAST FILE GENERATION ----
// ==============================

fn generate_should_fail_bin_wast(
    test_setup: &WastTestSetup,
    test_cases: &[WastTestCase],
    wast_path: &Path,
) -> Result<Vec<String>, std::io::Error> {
    let mut created_wast_files = vec![];
    for (test_idx, test_case) in test_cases.iter().enumerate() {
        for (assertion_idx, assertion) in test_case.assertions.iter().enumerate() {
            if assertion.passes_uninstrumented {
                continue;
            }
            // create the wast
            // call.wast -> call.idx.bin.wast
            let new_file_path = new_wast_path(
                wast_path,
                test_idx,
                Some(assertion_idx),
                OUTPUT_UNINSTR_WAST,
            );

            // Write new wast files, one assertion at a time
            write_bin_wast_file(
                &new_file_path,
                &test_setup.support_modules_wat,
                &test_setup.support_stmts,
                &test_setup.target_module_wat,
                &"None".to_string(),
                std::slice::from_ref(assertion),
            )?;
            created_wast_files.push(new_file_path);
        }
    }
    Ok(created_wast_files)
}

fn generate_instrumented_bin_wast(
    test_setup: &WastTestSetup,
    test_cases: &[WastTestCase],
    wast_path: &Path,
) -> Result<Vec<String>, std::io::Error> {
    let mut created_wast_files = vec![];
    for (idx, test_case) in test_cases.iter().enumerate() {
        // instrument A COPY OF the module with the whamm script
        // copy, so you don't accidentally manipulate the core module
        // (which is then instrumented in subsequent tests)
        let cloned_module = test_setup.target_module_wat.clone();
        let buff = wat::parse_bytes(cloned_module.as_slice())
            .expect("couldn't convert the input wat to Wasm");
        let mut module_to_instrument = Module::parse(&buff, false, true).unwrap();
        // make sure that this is a valid file by running wasm2wat through CLI
        let debug_file_path = format!(
            "{TEST_DEBUG_DIR}/{}.wasm",
            wast_path.file_name().unwrap().to_str().unwrap()
        );
        let wast_path_str = wast_path.to_str().unwrap().replace("\"", "");

        let core_lib = std::fs::read(CORE_WASM_PATH).unwrap_or_else(|_| {
            panic!(
                "Could not read the core wasm module expected to be at location: {}",
                CORE_WASM_PATH
            )
        });
        let def_yamls = pull_all_yml_files(DEFS_PATH);

        let mut metrics = Metrics::default();
        if let Err(mut err) = run(
            &core_lib,
            &def_yamls,
            &mut module_to_instrument,
            &test_case.whamm_script,
            &wast_path_str,
            vec![],
            0,
            &mut metrics,
            Config {
                as_monitor_module: false,
                enable_wei_alt: false,
                metrics: false,
                no_bundle: false,
                no_body: false,
                no_pred: false,
                no_report: false,
                testing: true,
                library_strategy: LibraryLinkStrategy::Imported,
            },
        ) {
            err.report();
            unreachable!("Shouldn't have had errors!")
        }

        let instrumented_module_wasm = module_to_instrument.encode();

        try_path(&debug_file_path);
        if let Err(e) = std::fs::write(&debug_file_path, instrumented_module_wasm.clone()) {
            unreachable!(
                "Failed to dump instrumented wasm to {} from error: {}",
                &debug_file_path, e
            )
        }
        wasm2wat_on_file(debug_file_path.as_str());

        // create the wast
        // call.wast -> call.idx.bin.wast
        let new_file_path = new_wast_path(wast_path, idx, None, OUTPUT_WHAMMED_WAST);

        write_bin_wast_file(
            &new_file_path,
            &test_setup.support_modules_wat,
            &test_setup.support_stmts,
            &instrumented_module_wasm,
            &test_case.whamm_script,
            &test_case.assertions,
        )?;
        created_wast_files.push(new_file_path);
    }
    Ok(created_wast_files)
}

fn write_bin_wast_file(
    file_path: &String,
    support_modules_wat: &Vec<Vec<u8>>,
    support_stmts: &Vec<String>,
    target_module: &Vec<u8>,
    whamm_script: &String,
    assertions: &[Assertion],
) -> Result<(), std::io::Error> {
    let mut wast_file = File::create(file_path)?;

    // output the support modules with format: (module binary "<binary>")
    for module in support_modules_wat {
        // wat2wasm
        let module_wasm = wat::parse_bytes(module).expect("couldn't convert the input wat to Wasm");

        wast_file.write_all("(module binary ".as_bytes())?;
        wast_file.write_all(vec_as_hex(module_wasm.as_ref()).as_bytes())?;
        wast_file.write_all(")\n\n".as_bytes())?;
    }

    // output the support statements
    for stmt in support_stmts {
        wast_file.write_all(stmt.as_bytes())?;
        wast_file.write_all(b"\n")?;
    }

    // output the target module binary with format: (module binary "<binary>")
    wast_file.write_all("(module binary ".as_bytes())?;
    wast_file.write_all(vec_as_hex(target_module.as_slice()).as_bytes())?;
    wast_file.write_all(")\n\n".as_bytes())?;

    // output the whamm script
    wast_file.write_all(format!("{} {}\n", WHAMM_PREFIX_PATTERN, whamm_script).as_bytes())?;

    // output the associated assertions (line by line)
    for assert in assertions.iter() {
        wast_file.write_all(assert.str.as_bytes())?;
        wast_file.write_all(b"\n")?;
    }
    wast_file.write_all(b"\n")?;
    wast_file
        .flush()
        .expect("Failed to flush out the wast file");

    Ok(())
}

// ==============================
// ---- TEST CASE COLLECTION ----
// ==============================

const WAST_SUITE_DIR: &str = "tests/wast_suite";
const MODULE_PREFIX_PATTERN: &str = "(module";
const ASSERT_PREFIX_PATTERN: &str = "(assert";
const WHAMM_PREFIX_PATTERN: &str = ";; WHAMM --> ";
const PASSES_UNINSTR_PATTERN: &str = ";; @passes_uninstr";
const TO_INSTR_PATTERN: &str = ";; @instrument";

/// Recursively finds all tests in a specified directory
pub(crate) fn find_wast_tests() -> Vec<PathBuf> {
    let mut wast_tests = Vec::new();
    let suite_path = Path::new(WAST_SUITE_DIR);

    find_tests(suite_path, &mut wast_tests);
    fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
        for f in path.read_dir().unwrap() {
            let f = f.unwrap();
            if f.file_type().unwrap().is_dir() {
                find_tests(&f.path(), tests);
                continue;
            }

            match f.path().extension().and_then(|s| s.to_str()) {
                Some("wast") => {} // found a test!
                Some("wasm") => panic!(
                    "use `*.wat` or `*.wast` instead of binaries: {:?}",
                    f.path()
                ),
                _ => continue,
            }
            tests.push(f.path());
        }
    }

    wast_tests
}

/// Holds the setup for a single test case encoded in the wast.
#[derive(Default)]
struct WastTestSetup {
    target_module_wat: Vec<u8>,
    support_modules_wat: Vec<Vec<u8>>,
    support_stmts: Vec<String>,
}

/// Parses the setup information from the wast file passed as a buffer.
/// This is necessary to support testing imports, e.g.:
/// (module
///   (func (export "log"))
/// )
/// (register "test")
/// ;; @instrument
/// (module <the actual targeted module to instrument>)
fn get_test_setup(
    reader: &mut BufReader<File>,
    file_path: &Path,
) -> Result<WastTestSetup, std::io::Error> {
    let mut mod_to_instr = false;

    let mut setup = WastTestSetup::default();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.starts_with(TO_INSTR_PATTERN) {
            mod_to_instr = true;
        } else if line.starts_with(MODULE_PREFIX_PATTERN) {
            // this is the beginning of the module
            let module = get_wasm_module(&line, reader)?;
            if mod_to_instr {
                if module.is_empty() {
                    panic!(
                        "Could not find the Wasm module-to-instrument in the wast file: {:?}",
                        file_path
                    );
                }

                debug!("{module}\n");
                setup.target_module_wat = Vec::from(module.as_bytes());
                // When we get to the target module, we know the setup is done!
                break;
            } else {
                setup.support_modules_wat.push(Vec::from(module.as_bytes()));
            }
            mod_to_instr = false;
        } else if line.starts_with('(') {
            setup.support_stmts.push(line.clone());
        }
        line.clear();
    }
    Ok(setup)
}

/// Parses the wasm module from the wast file passed as a buffer.
fn get_wasm_module(
    start_line: &str,
    reader: &mut BufReader<File>,
) -> Result<String, std::io::Error> {
    let mut module: String = start_line.to_string();
    let mut num_left_parens = count_matched_chars(&module, &'(');
    let mut num_right_parens = count_matched_chars(&module, &')');

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        // Add the line to the module string
        module += &line;

        // count the number of left/right parens (to know when finished parsing module)
        num_left_parens += count_matched_chars(&line, &'(');
        num_right_parens += count_matched_chars(&line, &')');

        if num_left_parens == num_right_parens {
            // we're done parsing the module!
            break;
        }

        line.clear();
    }
    fn count_matched_chars(s: &str, c: &char) -> usize {
        s.chars().filter(|ch| *ch == *c).count()
    }

    Ok(module)
}

/// Holds a single test case encoded in the wast.
#[derive(Default)]
struct WastTestCase {
    whamm_script: String,
    assertions: Vec<Assertion>,
}
impl WastTestCase {
    fn print(&self) {
        debug!(">>> TEST CASE <<<");
        debug!("{}", self.whamm_script);

        for assertion in &self.assertions {
            if assertion.passes_uninstrumented {
                debug!("PASS un-instrumented: '{}'", assertion.str);
            } else {
                debug!("FAIL un-instrumented: '{}'", assertion.str);
            }
        }
    }
}

#[derive(Clone)]
struct Assertion {
    str: String,
    passes_uninstrumented: bool,
}

/// Creates a vector of test cases from the passed buffer.
/// Convention: `whamm!` scripts are in comments beginning with "WHAMM --> "
/// Convention: All test cases under a `whamm!` script should be run on the same instrumented module.
fn get_test_cases(reader: BufReader<File>) -> Vec<WastTestCase> {
    let mut test_cases = Vec::new();

    let mut first = true;
    let mut matched = false;
    let mut passes_uninstr = false;
    let mut curr_test = WastTestCase::default();
    for line in reader.lines().map_while(Result::ok) {
        if let Some(whamm) = line.strip_prefix(WHAMM_PREFIX_PATTERN) {
            if !first {
                test_cases.push(curr_test);
                // this is the start of a new test case
                curr_test = WastTestCase::default();
            }
            first = false;
            matched = true;
            curr_test.whamm_script = whamm.to_string();
        } else if line.starts_with(MODULE_PREFIX_PATTERN) {
            panic!("Only one module per wast file!!")
        } else if line.starts_with(ASSERT_PREFIX_PATTERN) {
            // this is an assertion within the current test case
            curr_test.assertions.push(Assertion {
                str: line,
                passes_uninstrumented: passes_uninstr,
            });
            passes_uninstr = false;
        } else if line.starts_with(PASSES_UNINSTR_PATTERN) {
            passes_uninstr = true;
        }
    }
    if matched {
        // Make sure all tests are added!
        test_cases.push(curr_test);
    }

    test_cases
}

// ===================
// ---- UTILITIES ----
// ===================

fn new_wast_path(
    wast_path: &Path,
    idx: usize,
    idx2: Option<usize>,
    target_parent_dir: &str,
) -> String {
    // figure out name
    let file_name = wast_path.file_name().unwrap().to_str().unwrap().to_string();
    let file_ext = wast_path.extension().unwrap().to_str().unwrap();
    let file_name_stripped = file_name.strip_suffix(file_ext).unwrap();
    let new_name = if let Some(idx2) = idx2 {
        format!("{file_name_stripped}whamm{idx}.assertion{idx2}.bin.wast")
    } else {
        format!("{file_name_stripped}whamm{idx}.bin.wast")
    };

    // Figure out path
    let new_sub_path = match wast_path.strip_prefix(WAST_SUITE_DIR) {
        Ok(p) => p.to_str().unwrap(),
        Err(e) => panic!(
            "Could not strip prefix from path '{:?}' due to error: {:?}",
            wast_path, e
        ),
    };

    let new_path = format!("{target_parent_dir}/{}/{new_name}", new_sub_path);
    try_path(&new_path);

    new_path
}

/// Creates a String representing the &[u8] in hex format.
pub fn vec_as_hex(vec: &[u8]) -> String {
    // opening quote
    let mut res = "\"".to_string();

    // Iterate through each byte in the vector
    for &byte in vec {
        // Add each byte as a two-digit hexadecimal number with leading '\'
        res += format!("\\{:02x}", byte).as_str();
    }

    // closing quote
    res += "\"";
    res
}
