use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .include_item("UnrealBindings")
        .include_item("RustBindings")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file("../Ue5RustPlugin/Source/Ue5RustPlugin/Public/Bindings.h");
}