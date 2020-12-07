use std::path::Path;
use std::env;

fn main() {

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", dir);
    println!("cargo:rustc-link-lib=Gdi32");
    println!("cargo:rustc-link-lib=User32");
    println!("cargo:rustc-link-lib=dxgi");
    println!("cargo:rustc-link-search={}", Path::new(&dir).join("lib/Win32").display());
}
