use crate::test_instrumentation::helper::{run_core_suite, setup_tests};
use crate::util::setup_logger;

#[test]
fn instrument_with_paper_eval_branches_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/branches");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-branches", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_categories_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/categories");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-categories", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_hotness_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/hotness");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-hotness", processed_scripts, true, true)
}
#[test]
fn instrument_with_paper_eval_ins_count_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/ins_count");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-ins_count", processed_scripts, true, true)
}

#[test]
fn instrument_with_paper_eval_cache_sim_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("paper_eval/cache_sim");
    assert!(!processed_scripts.is_empty());

    run_core_suite("paper_eval-cache_sim", processed_scripts, true, true)
}
