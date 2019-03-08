# ew (EmacsWrapper)

`ew` is a thin wrapper of Emacs.

It executes `emacs` in server mode if no Emacs server is runnning.
Otherwise, executes `emacsclient`.

Works on Unix, macOS (Homebrew `emacsmac`) and Windows.

## Usage

### `ew` command

#### Options

The following options are handled by `ew`.
Rest arguments are passed to `emacs` or `emacsclient`.

* `-w`: Wait to finish edit.  
  If the option is specified, it does not pass `--no-wait` option to
  `emacsclient`.
  If Emacs server is not running, it starts Emacs server first and then
  executes `emacsclient`.
* `-m [DIR]`: Execute `magit-status` on directory DIR.

### `ew-w` command

This command is intended to be set as `EDITOR` environment variable.

It is just equivalent to `ew -w`, but suitable for `EDITOR` as some program
assumes `EDITOR` is a single command instead of shell command line.

