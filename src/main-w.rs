#![windows_subsystem = "windows"]

use std::env::args_os;

use ewrs::emacs::options::Options;

fn main() {
    let opts = Options::explicit(true, args_os().skip(1).collect());

    ewrs::run(&opts);
}
