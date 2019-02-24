pub mod emacs;
use emacs::common::Emacs;
use emacs::options::Options;
use emacs::OSEmacs;

pub fn run(opts: &Options) {
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
