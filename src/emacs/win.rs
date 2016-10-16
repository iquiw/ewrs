use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::mem;
use std::io::{Error, ErrorKind, Result};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;
use std::ptr;

use winapi::minwindef::{DWORD, FALSE};
use winapi::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::winuser::MB_OK;
use kernel32::{K32GetModuleFileNameExW, OpenProcess};
use user32::MessageBoxW;

use emacs::common::Emacs;

const EMACS_CMD: &'static str = "runemacs.exe";
const EMACSCLI_CMD: &'static str = "emacsclientw.exe";

pub struct WinEmacs {
}

impl<'a> Emacs<'a> for WinEmacs {
    fn new() -> Self { WinEmacs {} }

    fn emacs_cmd(&self) -> &'a str { EMACS_CMD }

    fn is_server_running(&self) -> Option<PathBuf> {
        read_pid_from_server_file()
            .and_then(|pid| {
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

    fn run_server<S>(&self, path: &Path, args: &[S]) -> Result<()> where S: AsRef<OsStr> {
        WinEmacs::run_server_cmd(path, args).map(|mut child| {
            if let Err(err) = child.wait() {
                WinEmacs::show_message(&format!("{}", err));
            }
        })
    }

    fn show_message(msg: &str) {
        let m = str_to_u16v(msg).as_ptr();
        let p = str_to_u16v("ew").as_ptr();
        unsafe {
            let _ = MessageBoxW(ptr::null_mut(), m, p, MB_OK);
        }
    }
}

fn str_to_u16v(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

const U_MAX_PATH: DWORD = 32767;

fn get_process_path(pid: DWORD) -> PathBuf {
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                            FALSE, pid);
        let mut v: [u16; U_MAX_PATH as usize] = mem::uninitialized();
        let nread = K32GetModuleFileNameExW(h,
                                            ptr::null_mut(),
                                            v.as_mut_ptr(),
                                            U_MAX_PATH);
        PathBuf::from(String::from_utf16_lossy(&v[0..(nread as usize)]))
    }
}

fn read_pid_from_server_file() -> Option<DWORD> {
    let home = env::home_dir().expect("HOME is not set");

    let mut p = PathBuf::from(home);
    p.push(".emacs.d");
    p.push("server");
    p.push("server");
    if p.is_file() {
        match read_pid(&p) {
            Ok(pid)  => Some(pid),
            Err(err) => {
                WinEmacs::show_message(&format!("{}", err));
                process::exit(1)
            }
        }
    } else {
        None
    }
}

fn read_pid<P>(p: P) -> Result<DWORD> where P: AsRef<Path> {
    let f = try!(File::open(p));
    let mut br = BufReader::new(f);
    let mut line = String::new();
    let _ = try!(br.read_line(&mut line));
    line.split_whitespace().nth(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No pid part"))
        .and_then(|s| s.parse().or_else(
            |_| Err(Error::new(ErrorKind::InvalidData, "Not a number"))))
}

#[test]
fn test_read_pid() {
    assert_eq!(read_pid("test/data/server").unwrap(), 6764);
}
