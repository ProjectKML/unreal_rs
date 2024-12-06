use std::fs;

fn main() {
    let include_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("RustPlugin")
        .join("Source")
        .join("RustPlugin")
        .join("Public");

    let bridge_h = include_dir.join("Bridge.h");

    fs::create_dir_all("gen").unwrap();

    bindgen::Builder::default()
        .header(bridge_h.to_str().unwrap())
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file("gen/bindings.rs")
        .expect("Failed to write bindings to file");
}