use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result, Write, stderr};
use std::path::{Path, PathBuf};

use libc::{fork, getuid, setsid};

use emacs::common::Emacs;

const EMACS_CMD: &'static str = "emacs";
const EMACSCLI_CMD: &'static str = "emacsclient";

pub struct UnixEmacs {
}

impl<'a> Emacs<'a> for UnixEmacs {
    fn new() -> Self { UnixEmacs {} }

    fn emacs_cmd(&self) -> &'a str { EMACS_CMD }

    fn is_server_running(&self) -> Option<PathBuf> {
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

    fn run_server<S>(&self, path: &Path, args: &[S]) -> Result<()> where S: AsRef<OsStr> {
        unsafe {
            let pid = fork();
            if pid > 0 {
                return Ok(());
            } else if pid < 0 {
                return Err(Error::new(ErrorKind::Other, "fork failed"));
            }
            let _ = setsid();
        }
        UnixEmacs::run_server_cmd(path, args).map(|_| ())
    }

    fn show_message(msg: &str) {
        let _ = writeln!(stderr(), "ew: {}", msg);
    }
}
