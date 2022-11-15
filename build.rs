extern crate bindgen;

extern crate cc;
extern crate pkg_config;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut cfg = cc::Build::new();
    let target = env::var("TARGET").unwrap();
    cfg.warnings(false);

    // if target.contains("windows") {
    //     cfg.define("_WIN32", None);
    //     cfg.define("BZ_EXPORT", None);
    // } else if !cfg!(feature = "static") {
    //     // pkg-config doesn't guarantee static link
    //     if pkg_config::Config::new()
    //         .cargo_metadata(true)
    //         .probe("bzip2")
    //         .is_ok()
    //     {
    //         return;
    //     }
    // }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    cfg.include("src/libstapsdt")
        .file("src/libstapsdt/src/dynamic-symbols.c")
        .file("src/libstapsdt/src/errors.c")
        .file("src/libstapsdt/src/hash-table.c")
        .file("src/libstapsdt/src/libstapsdt.c")
        .file("src/libstapsdt/src/sdtnote.c")
        .file("src/libstapsdt/src/section.c")
        .file("src/libstapsdt/src/shared-lib.c")
        .file("src/libstapsdt/src/string-table.c")
        .file("src/libstapsdt/src/util.c")
        .file("src/libstapsdt/src/asm/libstapsdt-x86_64.s")

        .out_dir(dst.join("lib"))
        .compile("libstapsdt.a");

    let src = env::current_dir().unwrap().join("src/libstapsdt");
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("src/libstapsdt.h"), dst.join("include/libstapsdt.h")).unwrap();
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());
    // // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.
    println!("cargo:rustc-link-lib=elf");
    println!("cargo:rustc-link-lib=dl");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
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

    let lib_dir = out_path.join("lib");
    println!("cargo:rustc‐link‐search=native={}", lib_dir.as_path().display().to_string()); //配置C库的搜索路径，相当于r
    // println!("cargo:rustc‐link‐search=native={}", "/home/steve/WorkSpace/ebpf-projs/rust-stapsdt/"); //配置C库的搜索路径，相当于r
    println!("cargo:rustc‐link‐lib=static=stapsdt"); //配置需要链接的C库名, 相当于rustc ‐l
}
