extern crate cc;

use std::{env, fs};
use std::path::PathBuf;

fn main() {
    let mut cfg = cc::Build::new();
    cfg.warnings(false);

    if env::var("TARGET").unwrap().contains("windows") {
        cfg.define("_WIN32", None);
        cfg.define("BZ_EXPORT", None);
    }

    cfg.include("bzip2-1.0.6").define("_FILE_OFFSET_BITS", Some("64"));
    if !cfg!(feature = "with-stdio") {
        cfg.define("BZ_NO_STDIO", None);
    };
    cfg.file("bzip2-1.0.6/blocksort.c")
       .file("bzip2-1.0.6/huffman.c")
       .file("bzip2-1.0.6/crctable.c")
       .file("bzip2-1.0.6/randtable.c")
       .file("bzip2-1.0.6/compress.c")
       .file("bzip2-1.0.6/decompress.c")
       .file("bzip2-1.0.6/bzlib.c")
       .compile("libbz2.a");

    let src = env::current_dir().unwrap().join("bzip2-1.0.6");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("bzlib.h"), dst.join("include/bzlib.h")).unwrap();
    println!("cargo:root={}", dst.display());
}
