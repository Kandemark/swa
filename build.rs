//! Build script — tells the linker where to find LLVM-C.lib.

fn main() {
    let llvm_prefix = std::env::var("LLVM_PREFIX")
        .unwrap_or_else(|_| "C:\\LLVM18".to_string());

    let lib_dir = format!("{}\\lib", llvm_prefix);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=LLVM-C");

    // Tell rustc where the DLL lives so it can be found at runtime during tests.
    let bin_dir = format!("{}\\bin", llvm_prefix);
    println!("cargo:rustc-link-search=native={}", bin_dir);
}
