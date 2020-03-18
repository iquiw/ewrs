use std::ffi::OsStr;
use std::ffi::OsString;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct ElispCommand {
    elisp: &'static str,
    dir: Option<OsString>,
}

const ELISP_OPTIONS: &[(&'static str, &'static str)] = &[("-m", "(magit-status)")];

#[derive(Debug, PartialEq)]
pub struct StandardOptions {
    pub wait: bool,
    pub args: Vec<OsString>,
}

#[derive(Debug, PartialEq)]
pub enum Options {
    Standard(StandardOptions),
    Elisp(ElispCommand),
    Help,
}

impl Options {
    pub fn explicit(wait: bool, args: Vec<OsString>) -> Self {
        Options::Standard(StandardOptions {
            wait: wait,
            args: args,
        })
    }

    pub fn parse<I, S>(args: I) -> Self
    where
        I: Iterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut wait = false;
        let mut rest = vec![];
        let v: Vec<S> = args.skip(1).collect();
        if v.iter().any(|arg| {
            let s = arg.as_ref();
            s == "--help" || s == "-h"
        }) {
            return Options::Help;
        }
        for i in 0..v.len() {
            let s = v[i].as_ref().to_os_string();
            if let Some(eopt) = ELISP_OPTIONS.iter().find(|x| s == x.0) {
                let dir = if i < v.len() - 1 {
                    Some(v[i + 1].as_ref().to_os_string())
                } else {
                    None
                };
                return Options::Elisp(ElispCommand {
                    elisp: eopt.1,
                    dir: dir,
                });
            }
            if s == "-w" {
                wait = true;
                continue;
            }
            rest.push(s);
        }
        Options::Standard(StandardOptions {
            wait: wait,
            args: rest,
        })
    }

    pub fn args(&self) -> &[OsString] {
        match self {
            Options::Standard(opts) => &opts.args,
            _ => &[],
        }
    }
}

pub trait CommandModifier {
    fn modify(&self, cmd: &mut Command);
}

impl CommandModifier for ElispCommand {
    fn modify(&self, cmd: &mut Command) {
        cmd.arg("-u").arg("-e").arg(format!(
            "(progn (select-frame-set-input-focus (selected-frame)) {})",
            self.elisp
        ));
        if let Some(dir) = &self.dir {
            cmd.current_dir(dir);
        }
    }
}

impl CommandModifier for StandardOptions {
    fn modify(&self, cmd: &mut Command) {
        let args = &self.args;
        if args.is_empty() {
            cmd.arg("-u")
                .arg("-e")
                .arg("(select-frame-set-input-focus (selected-frame))");
        } else {
            if !self.wait {
                cmd.arg("-n");
            }
            cmd.args(args);
        }
    }
}

impl CommandModifier for Options {
    fn modify(&self, cmd: &mut Command) {
        match self {
            Options::Standard(opts) => opts.modify(cmd),
            Options::Elisp(ec) => ec.modify(cmd),
            Options::Help => {}
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
            Options::Standard(StandardOptions {
                wait: false,
                args: vec![]
            })
        );
    }

    #[test]
    fn test_parse_prog_only() {
        let args: Vec<&str> = vec!["prog"];
        assert_eq!(
            Options::parse(args.iter()),
            Options::explicit(false, vec![])
        );
    }

    #[test]
    fn test_parse_prog_and_args() {
        let args: Vec<&str> = vec!["prog", "arg"];
        assert_eq!(
            Options::parse(args.iter()),
            Options::explicit(false, vec![OsString::from("arg")])
        );
    }

    #[test]
    fn test_parse_wait_true() {
        let args: Vec<&str> = vec!["prog", "-w"];
        assert_eq!(Options::parse(args.iter()), Options::explicit(true, vec![]));
    }

    #[test]
    fn test_parse_wait_true_and_args() {
        let args: Vec<&str> = vec!["prog", "-w", "arg1", "arg2"];
        assert_eq!(
            Options::parse(args.iter()),
            Options::explicit(true, vec![OsString::from("arg1"), OsString::from("arg2")])
        );
    }

    #[test]
    fn test_parse_elisp_magit_without_dir() {
        let args: Vec<&str> = vec!["prog", "-m"];
        assert_eq!(
            Options::parse(args.iter()),
            Options::Elisp(ElispCommand {
                elisp: "(magit-status)",
                dir: None,
            })
        )
    }

    #[test]
    fn test_parse_elisp_magit_with_dir() {
        let args: Vec<&str> = vec!["prog", "-m", "dir"];
        assert_eq!(
            Options::parse(args.iter()),
            Options::Elisp(ElispCommand {
                elisp: "(magit-status)",
                dir: Some(OsString::from("dir")),
            })
        )
    }

    #[test]
    fn test_parse_short_help() {
        let args: Vec<&str> = vec!["prog", "-h"];
        assert_eq!(Options::parse(args.iter()), Options::Help,)
    }

   #[test]
    fn test_parse_long_help() {
        let args: Vec<&str> = vec!["prog", "--help"];
        assert_eq!(Options::parse(args.iter()), Options::Help,)
    }
}
