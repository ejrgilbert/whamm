use crate::test_instrumentation::helper::{run_core_suite, setup_tests};
use crate::util::setup_logger;

#[test]
fn branches() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/branches");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-branches", processed_scripts, true, true)
}
#[test]
fn categories() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/categories");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-categories", processed_scripts, true, true)
}
#[test]
fn hotness() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/hotness");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-hotness", processed_scripts, true, true)
}
#[test]
fn ins_count() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/ins_count");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-ins_count", processed_scripts, true, true)
}

#[test]
fn cache_sim() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/cache_sim");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-cache_sim", processed_scripts, true, true)
}

#[test]
fn ins_coverage() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/ins_coverage");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-ins_coverage", processed_scripts, true, true)
}

#[test]
fn ins_coverage_dyninstr() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/ins_coverage_dyninstr");
    assert!(!processed_scripts.is_empty());

    // dyninstr isn't supported in bytecode rewriting, only the engine!
    run_core_suite(
        "paper_eval-ins_coverage_dyninstr",
        processed_scripts,
        false,
        true,
    )
}

#[test]
fn mem_access_tracing() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/mem_access_tracing");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-mem_access", processed_scripts, true, true)
}

#[test]
fn call_graph() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/call_graph");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-call_graph", processed_scripts, true, true)
}

#[test]
fn basic_block_profiling() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/basic_block_profiling");
    assert!(!processed_scripts.is_empty());

    run_core_suite(
        "paper_eval-basic_block_profiling",
        processed_scripts,
        true,
        false,
    )
}
