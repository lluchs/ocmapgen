extern crate cmake;
extern crate gcc;
extern crate regex;
extern crate glob;
extern crate bindgen;

use glob::glob;

fn main() {
    // Build libmisc and libc4script via cmake.
    let mut cmake_cfg = cmake::Config::new("openclonk");
    cmake_cfg.define("HEADLESS_ONLY", "ON");
    let cmake_dst = cmake_cfg.build_target("libmisc").build();
    cmake_cfg.build_target("libc4script").build();
    println!("cargo:rustc-link-search=native={}/build", cmake_dst.display());
    println!("cargo:rustc-link-lib=static=libmisc");
    println!("cargo:rustc-link-lib=static=libc4script");

    // Build glue code from mape manually.
    let mut cfg = gcc::Config::new();
    cfg.cpp(true);
    cfg.include("openclonk/src")
       .include("openclonk/include")
       .include("openclonk/thirdparty")
       .include(format!("{}/build", cmake_dst.display()));
    cfg.define("HAVE_CONFIG_H", Some("1"));

    for entry in glob("src/cpp-handles/*.cpp").unwrap() {
        if let Ok(f) = entry {
            cfg.file(f);
        }
    }
    cfg.compile("libcpphandles.a");

    // Generate ffi Rust bindings fo the cpp-handles header files.
    let bindings = bindgen::builder()
        .header("src/cpp-handles/bindgen.h")
        .whitelisted_function("c4_.+")
        .raw_line("#![allow(dead_code)]")
        .generate()
        .unwrap();
    bindings.write_to_file("src/ffi.rs").unwrap();
}
