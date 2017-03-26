extern crate cmake;
extern crate gcc;
extern crate regex;
extern crate glob;
extern crate bindgen;

use glob::glob;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    // Build libmisc and libc4script via cmake.
    let mut cmake_cfg = cmake::Config::new("openclonk");
    cmake_cfg.define("HEADLESS_ONLY", "ON");
    let cmake_dst = cmake_cfg.build_target("libmisc").build();
    cmake_cfg.build_target("libc4script").build();
    println!("cargo:rerun-if-changed=openclonk"); // note: will not apply to code changes inside

    // Build glue code from mape manually.
    let mut cfg = gcc::Config::new();
    cfg.cpp(true);
    cfg.include("openclonk/src")
       .include("openclonk/include")
       .include("openclonk/thirdparty")
       .include(format!("{}/build", cmake_dst.display()));
    cfg.define("HAVE_CONFIG_H", Some("1"));
    if env::var("PROFILE").unwrap() == "debug" {
        cfg.define("_DEBUG", Some("1"));
    }

    // Find file list from cmake.
    let cmakelists = read_file("openclonk/CMakeLists.txt").unwrap();
    let cmake_vars = get_cmake_vars(&cmakelists);
    for f in cmake_vars.get("MAPE_BASE_SOURCES").unwrap().iter()
                       .filter(|f| f.ends_with("cpp")) {
        cfg.file(format!("openclonk/{}", f));
    }

    for entry in glob("src/cpp-handles/*.cpp").unwrap() {
        if let Ok(f) = entry {
            println!("cargo:rerun-if-changed={}", f.display());
            cfg.file(f);
        }
    }
    cfg.compile("libcpphandles.a");

    println!("cargo:rustc-link-search=native={}/build", cmake_dst.display());
    println!("cargo:rustc-link-lib=static=libc4script");
    println!("cargo:rustc-link-lib=static=libmisc");
    println!("cargo:rustc-link-lib=z");

    // Generate ffi Rust bindings fo the cpp-handles header files.
    let bindings = bindgen::builder()
        .header("src/cpp-handles/bindgen.h")
        .whitelisted_function("c4_.+")
        .raw_line("#![allow(dead_code)]")
        .generate()
        .unwrap();
    bindings.write_to_file("src/ffi.rs").unwrap();

    for entry in glob("src/cpp-handles/*.h").unwrap() {
        if let Ok(f) = entry {
            println!("cargo:rerun-if-changed={}", f.display());
        }
    }
}
fn read_file(path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_cmake_vars(cmakelists: &str) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"(?xm)
        set\( (?P<var_name>\w+)
            (?P<files> (?: \n \s* [\w/.-]+ \s* $)+ )
    ").unwrap();
    let re_files = Regex::new(r"(?xm)
        \n \s* ([\w/.-]+) [[:blank:]]*
    ").unwrap();


    let mut result = HashMap::new();
    for caps in re.captures_iter(cmakelists) {
        let files: Vec<String> = re_files.captures_iter(&caps["files"])
            .map(|c| String::from(c.get(1).unwrap().as_str()))
            .collect();
        result.insert(caps["var_name"].to_string(), files);
    }

    result
}
