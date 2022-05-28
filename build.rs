extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/usr/include/chafa");

    // Tell cargo to tell rustc to link the system chafa
    // shared library.
    println!("cargo:rustc-link-lib=chafa");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // As per `pkg-config --cflags chafa`
        .clang_arg("-I/usr/include/chafa")
        .clang_arg("-I/usr/lib/chafa/include")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        .clang_arg("-I/usr/include/sysprof-4")

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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

