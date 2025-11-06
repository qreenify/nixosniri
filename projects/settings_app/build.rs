fn main() {
    // Print cargo instructions
    println!("cargo:rerun-if-changed=src/styles.css");
    println!("cargo:rerun-if-changed=build.rs");
}