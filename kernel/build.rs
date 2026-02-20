fn main() {
    // Force the kernel to be loaded on the higher half memory
    println!("cargo:rustc-link-arg=-Ttext=0xffffffff80000000");

    println!("cargo:rerun-if-changed=build.rs");
}
