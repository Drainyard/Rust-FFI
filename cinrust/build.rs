use std::path::Path;
use std::env;

fn main() {

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", Path::new(&dir).join("c/bin").display());
    println!("cargo:rustc-link-search={}", Path::new(&dir).join("lib").display());
    println!("cargo:rustc-link-lib=static=counter");
    println!("cargo:rustc-link-lib=static=User32");
}
