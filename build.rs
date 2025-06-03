use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Build the static library using the Makefile
    let status = Command::new("make")
        .arg("lib")
        .current_dir("../rayforce")
        .status()
        .expect("Failed to run make for lib");
    assert!(status.success(), "Make failed");

    // Tell cargo to link the pre-built static library
    println!("cargo:rustc-link-search=../rayforce");
    println!("cargo:rustc-link-lib=static=rayforce");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=../rayforce/core/rayforce.h");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type(".*")
        .allowlist_function(".*")
        .allowlist_var(".*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
