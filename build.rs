use glob::glob;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let cmakelists = read_file("openclonk/CMakeLists.txt").unwrap();
    // We need to patch the openclonk CMakeLists.txt slightly to make the build work.
    let cmakelists_patched = {
        macro_rules! comment_out {
            ($text:ident, $regex:literal) => {
                let $text = Regex::new($regex).unwrap()
                    .replace_all(&$text, "#$0");
            };
        }
        // Don't search for audio libraries to avoid having to specify the audio include path for
        // the glue code.
        comment_out!(cmakelists, r#"(?m)^\s*find_package\("Audio"\)$"#);
        // Image libraries aren't required, so remove that dependency.
        comment_out!(cmakelists, r#"(?m)^\s*find_package\((JPEG|PNG) REQUIRED\)$"#);
        comment_out!(cmakelists, r#"(?m)^\s*\$\{(JPEG|PNG)_INCLUDE_DIR\}$"#);
        // Mape's Log* symbols somehow override the ones in libmisc, but I can't get it to work
        // here. Just exclude the file instead.
        comment_out!(cmakelists, r#"(?m)^\s*src/lib/C4SimpleLog\.cpp$"#);
        // Don't require native c4group when cross-compiling.
        comment_out!(cmakelists, r#"(?m)^[^#\n]*IMPORT_NATIVE_TOOLS.*$"#);
        // We cannot compile with LTO enabled when linking with Rust.
        comment_out!(cmakelists, r#"(?m)^\s*set\(CMAKE_INTERPROCEDURAL_OPTIMIZATION_.*$"#);
        cmakelists.into_owned()
    };
    if cmakelists_patched != cmakelists {
        write_file("openclonk/CMakeLists.txt", &cmakelists_patched).unwrap();
    }

    // Build libmisc and libc4script via cmake.
    let mut cmake_cfg = cmake::Config::new("openclonk");
    cmake_cfg.define("HEADLESS_ONLY", "ON");
    let cmake_dst = cmake_cfg.build_target("libmisc").build();
    cmake_cfg.build_target("libc4script").build();
    cmake_cfg.build_target("blake2").build();
    println!("cargo:rerun-if-changed=openclonk"); // note: will not apply to code changes inside

    // Build glue code from mape manually.
    let mut cfg = cc::Build::new();
    cfg.cpp(true);
    cfg.include("openclonk/src")
       .include("openclonk/include")
       .include("openclonk/thirdparty")
       .include(format!("{}/build", cmake_dst.display()));
    cfg.define("HAVE_CONFIG_H", Some("1"));
    cfg.define("USE_CONSOLE", Some("1")); // avoid glew dependency
    if env::var("PROFILE").unwrap() == "debug" {
        cfg.define("_DEBUG", Some("1"));
    }

    // Find file list from cmake.
    let cmake_vars = get_cmake_vars(&cmakelists);
    for f in cmake_vars.get("MAPE_BASE_SOURCES").unwrap().iter()
                       .filter(|f| f.ends_with("cpp")) {
        cfg.file(format!("openclonk/{}", f));
    }
    cfg.file("openclonk/src/player/C4ScenarioParameters.cpp");

    for entry in glob("src/cpp-handles/*.cpp").unwrap() {
        if let Ok(f) = entry {
            println!("cargo:rerun-if-changed={}", f.display());
            cfg.file(f);
        }
    }
    cfg.compile("libcpphandles.a");

    println!("cargo:rustc-link-search=native={}/build", cmake_dst.display());
    println!("cargo:rustc-link-search=native={}/build/thirdparty/blake2", cmake_dst.display());
    println!("cargo:rustc-link-lib=static=libc4script");
    println!("cargo:rustc-link-lib=static=libmisc");
    println!("cargo:rustc-link-lib=static=blake2");
    println!("cargo:rustc-link-lib=z");

    if env::var("TARGET").unwrap().contains("windows") {
        println!("cargo:rustc-link-lib=winmm");
    }

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

fn write_file(path: &str, contents: &str) -> io::Result<()> {
    File::create(path)?
        .write_all(contents.as_bytes())
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
