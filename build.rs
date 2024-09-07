// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rustc-link-search=./include/");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
    }
}