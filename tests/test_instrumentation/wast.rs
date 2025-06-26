
#[test]
fn run_wast_tests() {
    crate::util::setup_logger();
    whamm::api::utils::run_wast_harness().expect("WAST Tests failed!");
}