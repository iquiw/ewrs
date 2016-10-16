pub mod common;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(unix)]
pub type OSEmacs = self::unix::UnixEmacs;
#[cfg(windows)]
pub type OSEmacs = self::win::WinEmacs;
