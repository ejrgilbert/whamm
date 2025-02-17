use std::cell::RefCell;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::slice;

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

                match Path::new(&outpath).try_exists() {
                    Ok(exists) => {
                        if !exists {
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
                    },
                    Err(e) => {
                        println!("Could not open file due to error: {:?}", e);
                        panic!("exiting...");
                    }
                }
            });
        }
    });
}

fn print(str: &str) {
    init();
    TO_CONSOLE.with(|to_console| {
        let to_console = to_console.borrow();
        if *to_console {
            print!("{str}");
        } else {
            OUTFILE.with(|outfile| {
                let Some(ref mut out) = &mut *outfile.borrow_mut() else {
                    panic!("No out file has been configured, please report this bug.");
                };

                // Write to a file
                out.write(str.as_bytes())
                    .expect("write failed");
            });
        }
    });
}

#[no_mangle]
pub fn putc(c: u8) {
    print(&String::from_utf8([c].to_vec()).expect("Our bytes should be valid utf8"));
}

#[no_mangle]
pub unsafe fn puts(start: i32, len: i32) {
    let ptr: *const u8 = start as *const u8;
    let s: &[u8] = unsafe { slice::from_raw_parts(ptr, usize::try_from(len).unwrap()) };

    let s = String::from_utf8(s.to_vec()).expect("Our bytes should be valid utf8");
    print(s.as_str());
}

#[no_mangle]
pub fn putu8(i: u8) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn puti8(i: i8) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn putu16(i: u16) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn puti16(i: i16) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn putu32(i: u32) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn puti32(i: i32) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn putu64(i: u64) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn puti64(i: i64) {
    print(&format!("{i}"));
}

#[no_mangle]
pub fn putf32(f: f32) {
    print(&format!("{:+e}", f));
}

#[no_mangle]
pub fn putf64(f: f64) {
    print(&format!("{:+e}", f));
}

#[no_mangle]
pub fn putbool(i: i32) {
    if i != 0 {
        print("true");
    } else {
        print("false");
    }
}
