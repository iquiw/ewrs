use std::io::{Error, ErrorKind, Result};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

pub trait Emacs {
    fn new() -> Self;

    fn emacs_cmd(&self) -> &str;

    fn find_path(&self) -> PathBuf {
        find_command_by_current_process()
            .and_then(|mut p| {
                p.push(self.emacs_cmd());
                if p.is_file() {
                    Some(p)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| PathBuf::from(self.emacs_cmd()))
    }

    fn is_server_running(&self) -> Option<PathBuf>;

    fn new_command<P: AsRef<OsStr>>(path: P) -> Command {
        Command::new(path)
    }

    fn run_client<S>(&self, path: &Path, args: &[S]) -> Result<()> where S: AsRef<OsStr> {
        let mut command = Self::new_command(PathBuf::from(path));
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

    fn run_server<S>(&self, path: &Path, args: &[S]) -> Result<()> where S: AsRef<OsStr>;

    fn run_server_cmd<S>(path: &Path, args: &[S]) -> Result<Child> where S: AsRef<OsStr> {
        let mut command = Self::new_command(path);
        command.arg("-f").arg("server-start").args(args).spawn()
    }

    fn show_message(msg: &str);
}

fn find_command_by_current_process() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|f| f.parent().map(|d| d.to_path_buf()))
}
