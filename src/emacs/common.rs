use std::env;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

use super::options::{CommandModifier, Options};

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

    fn run_client(&self, path: &Path, modifier: &CommandModifier) -> Result<()> {
        let mut command = Self::new_command(PathBuf::from(path));
        modifier.modify(&mut command);
        let status = command.status()?;
        if status.success() {
            Ok(())
        } else {
            status
                .code()
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::Interrupted,
                        format!("{}: process exited by signal", path.display()),
                    )
                })
                .and_then(|code| {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!("{}: process exited with code {}", path.display(), code),
                    ))
                })
        }
    }

    fn run_server(&self, path: &Path, opts: &Options) -> Result<()> {
        if opts.is_with_client() {
            self.run_server_os::<String>(&path, &[])?;
            let duration = Duration::from_secs(1);
            for _ in &[1..10] {
                thread::sleep(duration);
                if let Some(pathc) = self.is_server_running() {
                    return self.run_client(&pathc, opts);
                }
            }
            Err(Error::new(
                ErrorKind::Interrupted,
                "Timed out to wait for Emacs server",
            ))
        } else {
            self.run_server_os(&path, &opts.args())
        }
    }

    fn run_server_os<S>(&self, path: &Path, args: &[S]) -> Result<()>
    where
        S: AsRef<OsStr>;

    fn run_server_cmd<S>(path: &Path, args: &[S]) -> Result<Child>
    where
        S: AsRef<OsStr>,
    {
        let mut command = Self::new_command(path);
        let cmd = command.arg("-f").arg("server-start").args(args);
        if args.len() > 0 {
            cmd
        } else if let Some(home_dir) = dirs::home_dir() {
            cmd.current_dir(&home_dir)
        } else {
            cmd
        }
        .spawn()
    }

    fn show_message(msg: &str);
}

fn find_command_by_current_process() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|f| f.parent().map(|d| d.to_path_buf()))
}
