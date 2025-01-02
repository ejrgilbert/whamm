use std::cell::RefCell;
use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;

// ===============
// == CONSTANTS ==
// ===============

const DEFAULT_OUTDIR: &str = "output/whamm_core";
const DEFAULT_OUTFILE: &str = "whamm.out";

const TRUE: &str = "true";

// =========================
// == MUTABLE GLOBAL DATA ==
// =========================

thread_local! {
    static RAN_INIT: RefCell<bool> = RefCell::new(false);
    static TO_CONSOLE: RefCell<bool> = RefCell::new(false);
    static OUTPATH: RefCell<String> = RefCell::new("".to_string());
    static OUTFILE: RefCell<Option<File>> = RefCell::new(Option::default());
}

// ================
// == INIT FUNCS ==
// ================

fn init() {
    let mut ran = false;
    RAN_INIT.with(|ran_init| {
        let ran_init = &mut *ran_init.borrow_mut();

        if *ran_init {
            ran = true;
        } else {
            *ran_init = true;
        }
    });
    if ran { return }

    init_to_console();
    init_outfile();
}

fn init_to_console() {
    TO_CONSOLE.with(|to_console| {
        let to_console = &mut *to_console.borrow_mut();

        *to_console = match env::var("TO_CONSOLE") {
            Ok(val) => val == TRUE,
            Err(_) => false,
        };
    });
}

fn init_outfile() {
    TO_CONSOLE.with(|to_console| {
        let to_console = to_console.borrow();

        if !*to_console {
            let outdir = match env::var("WHAMM_OUTDIR") {
                Ok(val) => val,
                Err(_) => DEFAULT_OUTDIR.to_string(),
            };
            let _ = create_dir_all(outdir.as_str());

            let outfile = match env::var("WHAMM_OUTFILE") {
                Ok(val) => val,
                Err(_) => DEFAULT_OUTFILE.to_string(),
            };
            let outpath = format!("{}/{}", outdir, outfile);
            OUTPATH.with(|out| {
                let out = &mut *out.borrow_mut();
                *out = outpath.clone();
            });

            OUTFILE.with(|outfile| {
                let outfile = &mut *outfile.borrow_mut();

                if !Path::new(&outpath).exists() {
                    // create the outfile if it doesn't exist
                    *outfile = match File::create(&outpath) {
                        Err(why) => panic!("couldn't create {}: {}", outpath, why),
                        Ok(file) => Some(file),
                    };
                } else {
                    *outfile = Some(OpenOptions::new()
                        .append(true)
                        .open(outpath.clone())
                        .expect(format!("cannot open file at: {}", outpath).as_str()));
                }
            });
        }
    });
}

#[no_mangle]
pub fn putc(c: u8) {
    init();
    TO_CONSOLE.with(|to_console| {
        let to_console = to_console.borrow();
        if *to_console {
            let str = String::from_utf8([c].to_vec()).expect("Our bytes should be valid utf8");
            print!("{str}");
        } else {
            OUTFILE.with(|outfile| {
                let Some(ref mut out) = &mut *outfile.borrow_mut() else {
                    panic!("No out file has been configured, please report this bug.");
                };

                // Write to a file
                out.write(&[c])
                    .expect("write failed");
            });
        }
    });
}

#[no_mangle]
pub fn puti(i: i32) {
    init();
    TO_CONSOLE.with(|to_console| {
        let to_console = to_console.borrow();
        if *to_console {
            print!("{i}");
        } else {
            OUTFILE.with(|outfile| {
                let Some(ref mut out) = &mut *outfile.borrow_mut() else {
                    panic!("No out file has been configured, please report this bug.");
                };

                // Write to a file
                out.write(i.to_string().as_bytes())
                    .expect("write failed");
            });
        }
    });
    return;
}
