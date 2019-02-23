use std::ffi::OsStr;
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub struct Options {
    pub wait: bool,
    pub args: Vec<OsString>,
}

impl Options {
    pub fn parse<I, S>(args: I) -> Self
    where
        I: Iterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut wait = false;
        let mut rest = vec![];
        for arg in args.skip(1) {
            {
                let s = arg.as_ref().to_string_lossy();
                if s == "-w" {
                    wait = true;
                    continue;
                }
            }
            rest.push(arg.as_ref().to_os_string());
        }
        Options {
            wait: wait,
            args: rest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let args: Vec<&str> = vec![];
        assert_eq!(
            Options::parse(args.iter()),
            Options {
                wait: false,
                args: vec![]
            }
        );
    }

    #[test]
    fn test_parse_prog_only() {
        let args: Vec<&str> = vec!["prog"];
        assert_eq!(
            Options::parse(args.iter()),
            Options {
                wait: false,
                args: vec![]
            }
        );
    }

    #[test]
    fn test_parse_prog_and_args() {
        let args: Vec<&str> = vec!["prog", "arg"];
        assert_eq!(
            Options::parse(args.iter()),
            Options {
                wait: false,
                args: vec![OsString::from("arg")]
            }
        );
    }

    #[test]
    fn test_parse_wait_true() {
        let args: Vec<&str> = vec!["prog", "-w"];
        assert_eq!(
            Options::parse(args.iter()),
            Options {
                wait: true,
                args: vec![]
            }
        );
    }

    #[test]
    fn test_parse_wait_true_and_args() {
        let args: Vec<&str> = vec!["prog", "-w", "arg1", "arg2"];
        assert_eq!(
            Options::parse(args.iter()),
            Options {
                wait: true,
                args: vec![OsString::from("arg1"), OsString::from("arg2")]
            }
        );
    }
}
