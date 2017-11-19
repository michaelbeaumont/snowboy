extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();
    cc::Build::new()
        .cpp(true)
        .file("snowboy-detect-c-wrapper.cc")
        .flag_if_supported("-D_GLIBCXX_USE_CXX11_ABI=0")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-local-typedefs")
        .flag_if_supported("-Winit-self")
        .flag_if_supported("-DHAVE_POSIX_MEMALIGN")
        .flag_if_supported("-rdynamic")
        .compile("libsnowboy-detect-c-wrapper.a");
    let current_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir: PathBuf = current_dir.join(PathBuf::from("../../lib/ubuntu64"));
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=snowboy-detect");
    println!("cargo:rustc-link-lib=f77blas");
    println!("cargo:rustc-link-lib=cblas");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=atlas");
    println!("cargo:rustc-link-lib=asound");
}
