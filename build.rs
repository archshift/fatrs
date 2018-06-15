extern crate cc;
extern crate bindgen;

use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Write;

fn out_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn out_path_str() -> String {
    out_path().to_str().unwrap().to_owned()
}

fn write_extradefs(map: &HashMap<String, Option<String>>) {
    let mut file = File::create(out_path().join("extraopts.h")).unwrap();

    for (k, v) in map.iter() {
        if let &Some(ref v) = v {
            writeln!(file, "#define {} {}", k, v).unwrap();
        } else {
            writeln!(file, "#define {}", k).unwrap();
        }
    }
}

fn main() {
    let mut extra_defs = HashMap::<String, Option<String>>::new();

    if cfg!(feature = "unicode") {
        extra_defs.insert("_LFN_UNICODE".into(), Some("1".into()));
    } else {
        extra_defs.insert("_LFN_UNICODE".into(), Some("0".into()));
    };

    write_extradefs(&extra_defs);

    let bindings = bindgen::builder()
        .clang_arg(format!("-I{}", out_path_str()))
        .header("fatfs/ff.h")
        .header("fatfs/diskio.h")
        .use_core()
        .ctypes_prefix("::ctypes")
        .generate().unwrap();

    bindings.write_to_file(out_path().join("fatfs_bindings.rs")).unwrap();

    cc::Build::new()
        .file("fatfs/ff.c")
        .include("fatfs")
        .include(out_path())

        .flag("-nodefaultlibs")
        .flag("-fno-strict-aliasing")
        .compile("libfatfs.a");
}
