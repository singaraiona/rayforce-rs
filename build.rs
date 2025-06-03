use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Try to find Rayforce using pkg-config
    if let Ok(library_path) = pkg_config::Config::new()
        .probe("rayforce")
        .map(|lib| lib.link_paths[0].clone())
    {
        println!("cargo:rustc-link-search=native={}", library_path.display());
        println!("cargo:rustc-link-lib=rayforce");
    } else {
        // Fallback to checking common system library locations
        let system_lib_paths = ["/usr/lib", "/usr/local/lib", "/opt/rayforce/lib"];

        let mut found = false;
        for path in system_lib_paths {
            if std::path::Path::new(path).join("librayforce.so").exists()
                || std::path::Path::new(path).join("librayforce.a").exists()
            {
                println!("cargo:rustc-link-search=native={}", path);
                println!("cargo:rustc-link-lib=rayforce");
                found = true;
                break;
            }
        }

        if !found {
            panic!("Could not find Rayforce library. Please install it first.");
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

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
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
