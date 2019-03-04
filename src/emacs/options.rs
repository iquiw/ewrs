use std::ffi::OsStr;
use std::ffi::OsString;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct ElispCommand {
    opt: &'static str,
    name: &'static str,
    elisp: &'static str,
}

const ELISP_COMMANDS: &[&ElispCommand] = &[&ElispCommand {
    opt: "-m",
    name: "magit",
    elisp: "(magit-status)",
}];

#[derive(Debug, PartialEq)]
pub struct StandardOptions {
    pub wait: bool,
    pub args: Vec<OsString>,
}

#[derive(Debug, PartialEq)]
pub enum Options {
    Standard(StandardOptions),
    Elisp(&'static ElispCommand),
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
        for arg in args.skip(1) {
            {
                let s = arg.as_ref().to_string_lossy();
                if let Some(elisp) = ELISP_COMMANDS.iter().find(|x| s == x.opt) {
                    return Options::Elisp(elisp);
                }
                if s == "-w" {
                    wait = true;
                    continue;
                }
            }
            rest.push(arg.as_ref().to_os_string());
        }
        Options::Standard(StandardOptions {
            wait: wait,
            args: rest,
        })
    }

    pub fn is_with_client(&self) -> bool {
        match self {
            Options::Standard(opts) => opts.wait,
            _ => true,
        }
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
}
