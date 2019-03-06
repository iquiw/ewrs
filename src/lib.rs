pub mod emacs;
use emacs::common::Emacs;
use emacs::options::Options;
use emacs::OSEmacs;

fn help() {
    println!(
        "usage: ew [OPTION] [ARG ..]

  ew [emacs argument ..]
  ew -w [emacsclient argument ..]
  ew -m [DIR]
  ew (-h|--help)

options:
  -w           : Wait to finish editing.
  -h, --help   : Show this help.

elisp commands:
  -m [DIR]     : Run magit-status on directory DIR
"
    );
}

pub fn run(opts: &Options) {
    if opts == &Options::Help {
        help();
        return;
    }
    let emacs = OSEmacs::new();

    let result = match emacs.is_server_running() {
        Some(path) => emacs.run_client(&path, opts),
        None => {
            let path = emacs.find_path();
            emacs.run_server(&path, opts)
        }
    };
    if let Err(err) = result {
        OSEmacs::show_message(&format!("{}", err));
    }
}
