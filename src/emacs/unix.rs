use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use std::process;
use std::process::Command;

use libc::{fork,getuid,setsid};

pub const EMACS_CMD: &'static str = "emacs";
pub const EMACSCLI_CMD: &'static str = "emacsclient";

pub fn run_emacs<S>(path: &Path, args: &[S])
    where S: AsRef<OsStr> {
    unsafe {
        if fork() != 0 {
            return;
        }
        let _ = setsid();
    }
    let mut command = Command::new(path);
    let child = command.arg("-f").arg("server-start").args(args).spawn();
    if child.is_err() {
        process::exit(1);
    }
}

pub fn is_server_running() -> Option<PathBuf> {
    let mut path = PathBuf::from("/tmp");
    unsafe {
        path.push(format!("emacs{}", getuid()));
    }
    path.push("server");
    if path.exists() {
        Some(PathBuf::from(EMACSCLI_CMD))
    } else {
        None
    }
}
