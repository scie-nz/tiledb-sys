extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=dylib=tiledb");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=zstd");
    println!("cargo:rustc-link-lib=lz4");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-flags=-ltiledb");

    println!("cargo:rustc-link-search=/home/bogdan/TileDB/dist/lib");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .rustfmt_bindings(true)
        .clang_args(vec![
            "-I/home/bogdan/TileDB/dist/include",
            "-std=c++11",
        ])
        .size_t_is_usize(false)
        .enable_cxx_namespaces()
        .opaque_type("std::.*")
        //.opaque_type("tiledb::VFSFilebuf.*")
        .module_raw_lines(
            "root",
            [
                "pub type int_type = ::std::os::raw::c_int;",
                "pub type off_type = root::std::streamoff;",
            ]
            .iter()
            .map(|s| *s),
        )
        .opaque_type("tiledb::impl_::ConfigIter")
        .opaque_type("tiledb::ObjectIter_iterator")
        .blocklist_type("int_type")
        .blocklist_type("off_type")
        .ignore_methods()
        .opaque_type(".*VFSFilebuf.*")
        .allowlist_type("tiledb.*")
        .blocklist_type("tiledb::VFSFilebuf.*")
        .opaque_type(".*VFS_filebuf.*")
        .blocklist_type(".*ObjectIter_iterator_iterator1.*")
        .generate_inline_functions(true)
        .rustified_enum(".*")
        .new_type_alias(".*")
        .derive_copy(false)
        .allowlist_function("tiledb.*")
        .allowlist_var("tiledb.*")
        .allowlist_function("tiledb::Context.*")
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
