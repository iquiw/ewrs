#![windows_subsystem = "windows"]

#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winapi;

extern crate dirs;

use std::ffi::OsString;

mod emacs;
use emacs::common::Emacs;
use emacs::OSEmacs;

fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

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
