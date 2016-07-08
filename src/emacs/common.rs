use std::io::{stderr,Write};
use std::env;
use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use std::process;
use std::process::Command;

use emacs::EMACS_CMD;

macro_rules! die {
    ($fmt:expr) => {{
        let _ = writeln!(stderr(), $fmt);
        process::exit(1);
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let _ = writeln!(stderr(), $fmt, $($arg)*);
        process::exit(1);
    }};
}

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
        .unwrap_or(PathBuf::from(EMACS_CMD))
}

pub fn run_emacscli<S>(path: &Path, args: &[S]) where S: AsRef<OsStr> {
    let mut command = Command::new(PathBuf::from(path));
    if args.is_empty() {
        command
            .arg("-e")
            .arg("(select-frame-set-input-focus (selected-frame))");
    } else {
        command.arg("-n").args(args);
    }
    let result = command.status();
    match result {
        Ok(status) => {
            if !status.success() {
                match status.code() {
                    Some(code) =>
                        die!("{}: process exited with code {}",
                             path.display(), code),
                    None =>
                        die!("{}: process exited by signal",
                             path.display())
                }
            }
        },
        Err(err) => {
            die!("{}: {}", path.display(), err);
        }
    }
}

