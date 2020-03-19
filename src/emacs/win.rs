use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind, Result};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};
use std::ptr;

use widestring::WideCString;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::K32GetModuleFileNameExW;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::MessageBoxW;
use winapi::um::winuser::MB_OK;

use super::common::Emacs;

const EMACS_CMD: &'static str = "runemacs.exe";
const EMACSCLI_CMD: &'static str = "emacsclientw.exe";

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
            let path = get_process_path(pid);
            path.file_name()
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
                })
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
        let m = str_to_widec(msg).into_raw();
        let p = str_to_widec("ew").into_raw();
        unsafe {
            let _ = MessageBoxW(ptr::null_mut(), m, p, MB_OK);
        }
    }
}

fn str_to_widec(s: &str) -> WideCString {
    WideCString::from_str(s).expect("Message contains nul")
}

const U_MAX_PATH: DWORD = 32767;

fn get_process_path(pid: DWORD) -> PathBuf {
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
        let mut v: [u16; U_MAX_PATH as usize] = MaybeUninit::uninit().assume_init();
        let nread = K32GetModuleFileNameExW(h, ptr::null_mut(), v.as_mut_ptr(), U_MAX_PATH);
        PathBuf::from(String::from_utf16_lossy(&v[0..(nread as usize)]))
    }
}

fn read_pid_from_server_file() -> Option<DWORD> {
    let home = dirs::home_dir().expect("HOME is not set");

    let mut p = PathBuf::from(home);
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

fn read_pid<P>(p: P) -> Result<DWORD>
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
                .or_else(|_| Err(Error::new(ErrorKind::InvalidData, "Not a number")))
        })
}

#[test]
fn test_read_pid() {
    assert_eq!(read_pid("test/data/server").unwrap(), 6764);
}
