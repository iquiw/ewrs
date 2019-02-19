#![windows_subsystem = "windows"]

#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winapi;

extern crate dirs;

use std::env::{args_os, ArgsOs};

mod emacs;
use emacs::common::{Emacs, Options};
use emacs::OSEmacs;

impl Options {
    fn parse(args: ArgsOs) -> Self {
        let mut wait = false;
        let mut rest = vec![];
        for arg in args.skip(1) {
            {
                let s = arg.to_string_lossy();
                if s == "-w" {
                    wait = true;
                    continue;
                }
            }
            rest.push(arg);
        }
        Options {
            wait: wait,
            args: rest,
        }
    }
}

fn main() {
    let opts = Options::parse(args_os());

    let emacs = OSEmacs::new();

    let result = match emacs.is_server_running() {
        Some(path) => emacs.run_client(&path, &opts),
        None => {
            let path = emacs.find_path();
            emacs.run_server(&path, &opts.args)
        }
    };
    if let Err(err) = result {
        OSEmacs::show_message(&format!("{}", err));
    }
}
