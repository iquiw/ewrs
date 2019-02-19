use std::env::ArgsOs;
use std::ffi::OsString;

pub struct Options {
    pub wait: bool,
    pub args: Vec<OsString>,
}

impl Options {
    pub fn parse(args: ArgsOs) -> Self {
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
