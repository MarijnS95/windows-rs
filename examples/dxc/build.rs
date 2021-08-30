fn main() {
    // let p = env!("CARGO_MANIFEST_DIR");
    let p = ".";
    println!("cargo:rustc-link-search=native={}", p);
    // println!("cargo:rustc-link-lib=dxcompiler");
}
