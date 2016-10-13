use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result, Write, stderr};
use std::path::{Path, PathBuf};
use std::process::Command;

use libc::{fork, getuid, setsid};

pub const EMACS_CMD: &'static str = "emacs";
pub const EMACSCLI_CMD: &'static str = "emacsclient";

pub fn run_emacs<S>(path: &Path, args: &[S]) -> Result<()>
    where S: AsRef<OsStr> {
    unsafe {
        let pid = fork();
        if pid > 0 {
            return Ok(());
        } else if pid < 0 {
            return Err(Error::new(ErrorKind::Other, "fork failed"));
        }
        let _ = setsid();
    }
    let mut command = Command::new(path);
    command.arg("-f").arg("server-start").args(args).spawn()
        .map(|_| ())
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

pub fn show_message(msg: &str) {
    let _ = writeln!(stderr(), "ew: {}", msg);
}
