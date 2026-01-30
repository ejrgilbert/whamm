use std::fs;
use std::path::PathBuf;

pub fn pull_all_yml_files(base_dir_tmp: &str) -> Vec<String> {
    let base_dir = base_dir_tmp.trim_end_matches("/");

    let mut yml_files = vec![];

    // push events first (sets up the anchors)
    pull_yml(
        &mut yml_files,
        &format!("{base_dir}/providers/packages/events"),
    );
    // push packages next (sets up the anchors)
    pull_yml(&mut yml_files, &format!("{base_dir}/providers/packages"));
    // finally the providers
    pull_yml(&mut yml_files, &format!("{base_dir}/providers"));

    if yml_files.is_empty() {
        panic!(
            "[ERROR] Could not load provider definitions from base directory: {}\n\tPlease make sure you follow the expected directory structure!\n\tExiting now...",
            base_dir
        );
    }
    yml_files
}

fn pull_yml(files: &mut Vec<String>, path: &str) {
    pull_glob(files, &format!("{path}/*.yml"));
    pull_glob(files, &format!("{path}/*.yaml"));
}

fn pull_glob(files: &mut Vec<String>, glob: &str) {
    for path in glob::glob(glob).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        process_file(files, file_name);
    }
}

fn process_file(files: &mut Vec<String>, file_name: &PathBuf) {
    let unparsed_file = fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &file_name));
    files.push(unparsed_file);
}
