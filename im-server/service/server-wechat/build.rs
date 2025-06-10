fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("musl") {
        println!("cargo:rustc-link-lib=m");
    }
}
