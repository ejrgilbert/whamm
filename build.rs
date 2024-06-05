use clap::CommandFactory;
use clap_mangen::Man;
use project_root::get_project_root;
use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::exit;

include!("src/cli.rs");

fn build_man(out_dir: &Path) -> Result<(), Error> {
    let app = WhammCli::command();

    let file = Path::new(&out_dir).join("example.1");
    let mut file = File::create(file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    // Create `target/assets/` folder.
    let mut path = match get_pb(&PathBuf::from("target")) {
        Ok(pb) => pb,
        Err(_) => exit(1),
    };
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    // build_shell_completion(&path)?;
    build_man(&path)?;

    Ok(())
}

fn get_pb(file_pb: &PathBuf) -> Result<PathBuf, String> {
    if file_pb.is_relative() {
        match get_project_root() {
            Ok(r) => {
                let mut full_path = r.clone();
                full_path.push(file_pb);
                Ok(full_path)
            }
            Err(e) => Err(format!("the root folder does not exist: {:?}", e)),
        }
    } else {
        Ok(file_pb.clone())
    }
}
