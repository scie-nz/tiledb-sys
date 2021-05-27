extern crate bindgen;
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut options = CopyOptions::new();
    options.skip_exist = true;
    options.content_only = true;
    copy("TileDB", &out_dir, &options).unwrap();
    let current_dir = env::current_dir().unwrap();
    Command::new("mkdir")
        .arg("-p")
        .arg(format!("{}/build", &out_dir))
        .arg(".")
        .status()
        .expect("failed to run bootstrap");
    assert!(env::set_current_dir(Path::new(&format!("{}/build", &out_dir))).is_ok());
    Command::new("../bootstrap")
        .arg("--enable-s3")
        .status()
        .expect("failed to run bootstrap");
    Command::new("make")
        .arg("-j")
        .arg("12")
        .status()
        .expect("failed to run make");
    Command::new("make")
        .arg("install-tiledb")
        .status()
        .expect("failed to run make");
    assert!(env::set_current_dir(current_dir).is_ok());

    // Tell cargo to tell rustc to link the tiledb library.
    println!("cargo:rustc-link-lib=dylib=tiledb");
    // search for the library in a custom location
    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/dist/lib", &out_dir),
    );

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .rustified_enum(".*")
        .generate_inline_functions(true)
        .clang_args(vec![
            format!("-I{}/dist/include", &out_dir),
            "-std=c++11".to_string(),
        ])
        .enable_cxx_namespaces()
        .opaque_type("std::.*")
        .blocklist_type("int_type")
        .blocklist_type("off_type")
        .allowlist_type("tiledb.*")
        .module_raw_lines(
            "root",
            [
                "pub type int_type = ::std::os::raw::c_int;",
                "pub type off_type = root::std::streamoff;",
            ]
            .iter()
            .map(|s| *s),
        )
        .derive_copy(false)
        .allowlist_function("tiledb.*")
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
