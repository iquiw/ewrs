#![windows_subsystem = "windows"]

use std::env::args_os;

use ewrs::emacs::options::Options;

fn main() {
    let opts = Options::parse(args_os());

    ewrs::run(&opts);
}
