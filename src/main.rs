#![windows_subsystem = "windows"]

#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate widestring;

use std::ffi::OsString;

mod emacs;
use emacs::OSEmacs;
use emacs::common::Emacs;

fn main() {
    let args: Vec<OsString> = std::env::args_os()
        .skip(1)
        .collect();

    let emacs = OSEmacs::new();

    let result = match emacs.is_server_running() {
        Some(path) => emacs.run_client(&path, &args[..]),
        None => {
            let path = emacs.find_path();
            emacs.run_server(&path, &args[..])
        }
    };
    if let Err(err) = result {
        OSEmacs::show_message(&format!("{}", err));
    }
}
