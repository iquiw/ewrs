use std::cmp::Ordering;
use std::ffi::OsStr;
use std::io::{stderr, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

use dirs::runtime_dir;
use libc::{fork, getuid, setsid};

use super::common::Emacs;

const EMACS_CMD: &str = "emacs";
const EMACSCLI_CMD: &str = "emacsclient";

pub struct UnixEmacs {}

impl UnixEmacs {
    #[cfg(feature = "emacs27")]
    fn emacs_server_dir() -> PathBuf {
        if let Some(mut dir) = runtime_dir() {
            dir.push("emacs");
            dir
        } else {
            let mut path = PathBuf::from("/tmp");
            unsafe {
                path.push(format!("emacs{}", getuid()));
            }
            path
        }
    }

    #[cfg(not(feature = "emacs27"))]
    fn emacs_server_dir() -> PathBuf {
        let mut path = PathBuf::from("/tmp");
        unsafe {
            path.push(format!("emacs{}", getuid()));
        }
        path
    }
}

impl Emacs for UnixEmacs {
    fn new() -> Self {
        UnixEmacs {}
    }

    fn emacs_cmd(&self) -> &str {
        EMACS_CMD
    }

    fn is_server_running(&self) -> Option<PathBuf> {
        let mut path = UnixEmacs::emacs_server_dir();
        path.push("server");
        if path.exists() {
            Some(PathBuf::from(EMACSCLI_CMD))
        } else {
            None
        }
    }

    fn run_server_os<S>(&self, path: &Path, args: &[S]) -> Result<()>
    where
        S: AsRef<OsStr>,
    {
        unsafe {
            let pid = fork();
            match pid.cmp(&0) {
                Ordering::Greater => return Ok(()),
                Ordering::Less => return Err(Error::new(ErrorKind::Other, "fork failed")),
                Ordering::Equal => {}
            }
            let _ = setsid();
        }
        UnixEmacs::run_server_cmd(path, args).map(|_| ())?;
        std::process::exit(0);
    }

    fn show_message(msg: &str) {
        let _ = writeln!(stderr(), "ew: {}", msg);
    }
}
