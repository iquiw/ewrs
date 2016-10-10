#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate user32;

use std::ffi::OsString;

mod emacs;

fn main() {
    let args: Vec<OsString> = std::env::args_os()
        .skip(1)
        .collect();

    let result = match emacs::is_server_running() {
        Some(path) => {
            emacs::run_emacscli(&path, &args[..])
        },
        None => {
            let path = emacs::find_emacs();
            emacs::run_emacs(&path, &args[..])
        }
    };
    if let Err(err) = result {
        emacs::show_message(&format!("{}", err));
    }
}
