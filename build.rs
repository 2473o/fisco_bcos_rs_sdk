fn main() {
    let path = ".";
    println!("cargo:rustc-link-search={}", path);
}
