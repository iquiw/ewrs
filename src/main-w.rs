#![windows_subsystem = "windows"]

use std::env::args_os;

use ewrs::emacs::options::Options;

fn main() {
    let mut opts = Options::default();
    opts.wait = true;
    opts.args = args_os().skip(1).collect();

    ewrs::run(&opts);
}
