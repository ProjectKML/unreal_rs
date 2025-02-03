use std::fs;

fn main() {
    let include_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Source");

    let bridge_h = include_dir.join("Bridge.h");

    fs::create_dir_all("gen").unwrap();

    bindgen::Builder::default()
        .header(bridge_h.to_str().unwrap())
        .layout_tests(false)
        .allowlist_function("ur_.*")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file("gen/bindings.rs")
        .expect("Failed to write bindings to file");
}
