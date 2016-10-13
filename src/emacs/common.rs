use std::io::{Error, ErrorKind, Result};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use emacs::EMACS_CMD;

fn find_command_by_current_process() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|f| f.parent().map(|d| d.to_path_buf()))
}

pub fn find_emacs() -> PathBuf {
    find_command_by_current_process()
        .and_then(|mut p| {
            p.push(EMACS_CMD);
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from(EMACS_CMD))
}

pub fn run_emacscli<S>(path: &Path, args: &[S]) -> Result<()> where S: AsRef<OsStr> {
    let mut command = Command::new(PathBuf::from(path));
    if args.is_empty() {
        command
            .arg("-e")
            .arg("(select-frame-set-input-focus (selected-frame))");
    } else {
        command.arg("-n").args(args);
    }
    let status = try!(command.status());
    if status.success() {
        Ok(())
    } else {
        status.code()
            .ok_or_else(|| Error::new(ErrorKind::Interrupted,
                                      format!("{}: process exited by signal",
                                              path.display())))
            .and_then(|code| {
                Err(Error::new(ErrorKind::Other,
                               format!("{}: process exited with code {}",
                                       path.display(), code)))
            })
    }
}

