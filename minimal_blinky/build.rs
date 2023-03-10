//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::path::PathBuf;

fn reference() {
    let out: &PathBuf = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:warning={:?}", out.display());
}

fn linker_script() {
    println!("cargo:rerun-if-changed=gcc_arm.ld");
}

fn main() {
    reference();
    linker_script();
}
