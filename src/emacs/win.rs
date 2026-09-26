use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};

use windows::core::{HSTRING, PCWSTR};
use windows::Win32::System::Threading::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, OpenProcess};
use windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;

use super::common::Emacs;

const EMACS_CMD: &str = "runemacs.exe";
const EMACSCLI_CMD: &str = "emacsclientw.exe";

pub struct WinEmacs {}

impl Emacs for WinEmacs {
    fn new() -> Self {
        WinEmacs {}
    }

    fn emacs_cmd(&self) -> &str {
        EMACS_CMD
    }

    fn is_server_running(&self) -> Option<PathBuf> {
        read_pid_from_server_file().and_then(|pid| {
            let path_opt = get_process_path(pid);
            path_opt.and_then(|path| path.file_name()
                .and_then(|name| {
                    if name == "emacs.exe" {
                        path.parent()
                    } else {
                        None
                    }
                })
                .map(|p| {
                    let mut pb = p.to_path_buf();
                    pb.push(EMACSCLI_CMD);
                    pb
                }))
        })
    }

    fn new_command<P: AsRef<OsStr>>(path: P) -> Command {
        let mut command = Command::new(path);
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        command
    }

    fn run_server_os<S>(&self, path: &Path, args: &[S]) -> Result<()>
    where
        S: AsRef<OsStr>,
    {
        WinEmacs::run_server_cmd(path, args).map(|mut child| {
            if let Err(err) = child.wait() {
                WinEmacs::show_message(&format!("{}", err));
            }
        })
    }

    fn show_message(msg: &str) {
        let m = HSTRING::from(msg);
        let p = HSTRING::from("ew");
        unsafe {
            let _ = MessageBoxW(None, PCWSTR::from_raw(m.as_ptr()), PCWSTR::from_raw(p.as_ptr()), MB_OK);
        }
    }
}

const U_MAX_PATH: usize = 32767;

fn get_process_path(pid: u32) -> Option<PathBuf> {
    unsafe {
        let result = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if let Ok(h) = result {
            let mut v: Vec<u16> = vec![0; U_MAX_PATH];
            let nread = GetModuleFileNameExW(Some(h), None, &mut v);
            if nread > 0 {
                v.set_len(nread as usize);
                return Some(PathBuf::from(String::from_utf16_lossy(&v)));
            }
        }
        None
    }
}

fn read_pid_from_server_file() -> Option<u32> {
    let mut p = dirs::home_dir().expect("HOME is not set");

    p.push(".emacs.d");
    p.push("server");
    p.push("server");
    if p.is_file() {
        match read_pid(&p) {
            Ok(pid) => Some(pid),
            Err(err) => {
                WinEmacs::show_message(&format!("{}", err));
                process::exit(1)
            }
        }
    } else {
        None
    }
}

fn read_pid<P>(p: P) -> Result<u32>
where
    P: AsRef<Path>,
{
    let f = File::open(p)?;
    let mut br = BufReader::new(f);
    let mut line = String::new();
    let _ = br.read_line(&mut line)?;
    line.split_whitespace()
        .nth(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No pid part"))
        .and_then(|s| {
            s.parse()
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Not a number"))
        })
}

#[test]
fn test_read_pid() {
    assert_eq!(read_pid("test/data/server").unwrap(), 6764);
}
