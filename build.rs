use std::env;
use std::path::Path;
use std::process::Command;
use std::env::consts::{ARCH, OS};

fn main() {
    if OS != "windows" {
        return;
    }
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    Command::new("windres")
        .arg("-O").arg("coff")
        .arg("-F").arg(if ARCH == "x86_64" { "pe-x86-64" } else { "pe-i386" })
        .arg("-i").arg("ewrs.rc")
        .arg("-o").arg(&format!("{}/ewrs.res", out_dir))
        .status().unwrap();
    Command::new("ar")
        .arg("crs").arg("libewrs_rc.a")
        .arg("ewrs.res")
        .current_dir(Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ewrs_rc");
}
