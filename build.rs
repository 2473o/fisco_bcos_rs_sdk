fn main() {
    let home = env!("HOME");
    let lib_path = format!("{}/FiscoBCOS/local/lib", home);

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-link-lib=dylib=bcos-c-sdk");
    }
}
