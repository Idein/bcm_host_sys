use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to tell rustc to link the system bcm_host shared library.
    let bcm_host = pkg_config::probe_library("bcm_host").unwrap();

    // Path to directories of C header
    let include_dirs: Vec<_> = vec![
        Path::new(&env::var("LIBCLANG_INCLUDE_PATH")
            .expect("LIBCLANG_INCLUDE_PATH like: '/usr/include/clang/3.9.1/include'"))
            .into(),
    ];

    let include_args: Vec<_> = include_dirs
        .iter()
        .chain(bcm_host.include_paths.iter())
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
        .clang_args(&include_args)
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
