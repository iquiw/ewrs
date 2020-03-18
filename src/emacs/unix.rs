use std::ffi::OsStr;
use std::io::{stderr, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

use libc::{fork, getuid, setsid};

use super::common::Emacs;

const EMACS_CMD: &'static str = "emacs";
const EMACSCLI_CMD: &'static str = "emacsclient";

pub struct UnixEmacs {}

impl Emacs for UnixEmacs {
    fn new() -> Self {
        UnixEmacs {}
    }

    fn emacs_cmd(&self) -> &str {
        EMACS_CMD
    }

    fn is_server_running(&self) -> Option<PathBuf> {
        #[cfg(feature = "emacs27")]
        let mut path = if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR") {
            PathBuf::from(dir)
        } else {
            PathBuf::from("/tmp")
        };
        #[cfg(not(feature = "emacs27"))]
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

    fn run_server_os<S>(&self, path: &Path, args: &[S]) -> Result<()>
    where
        S: AsRef<OsStr>,
    {
        unsafe {
            let pid = fork();
            if pid > 0 {
                return Ok(());
            } else if pid < 0 {
                return Err(Error::new(ErrorKind::Other, "fork failed"));
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
