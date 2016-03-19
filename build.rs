extern crate gcc;

fn main() {
    let mut cfg = gcc::Config::new();
    let mut cfg = cfg.file("fatfs/ff.c")
        .include("fatfs")
        .flag("-fno-strict-aliasing");

    let mut cfg = if cfg!(feature = "unicode") {
        cfg.define("_LFN_UNICODE", Some("1"))
            .file("fatfs/option/unicode.c")
    } else {
        cfg.define("_LFN_UNICODE", Some("0"))
    };

    cfg.compile("libfatfs.a");
}
