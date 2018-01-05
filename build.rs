extern crate bindgen;
extern crate pkg_config;
extern crate log;
extern crate env_logger;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to tell rustc to link the system bcm_host shared library.
    let bcm_host = pkg_config::probe_library("bcm_host").unwrap();

    let bcm_host_args: Vec<_> = bcm_host.include_paths.iter()
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();

    // Path to directories of Clang header
    let clang_dirs: Vec<PathBuf> = Vec::from(vec![
        Path::new(&env::var("CLANG_INCLUDE_DIR").expect(
            "CLANG_INCLUDE_DIR like: /usr/lib/llvm-3.9/lib/clang/3.9.1/include",
        )).into(),
    ]);

    let clang_args: Vec<_> = clang_dirs.iter()
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // derive from `pkg-config --cflags bcm_host`
        .clang_args(&["-D", "USE_VCHIQ_ARM"])
        .clang_args(&bcm_host_args)
        .clang_args(&clang_args)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
