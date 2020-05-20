const LINKER_SCRIPT: &str = "memory.x";

fn main() {
    // Put the linker script somewhere the linker can find it
    let src_dir = std::path::PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("Failed to locate project root directory"),
    );
    let link_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").expect("Failed to locate project build directory"),
    );
    std::fs::copy(src_dir.join(LINKER_SCRIPT), link_dir.join(LINKER_SCRIPT))
        .expect("Failed copying linker script from project root to build directory");
    println!("cargo:rustc-link-search={}", link_dir.to_string_lossy());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("{}", format!("cargo:rerun-if-changed={}", LINKER_SCRIPT));
}
