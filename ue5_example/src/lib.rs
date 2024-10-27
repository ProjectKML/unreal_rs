use ue5::prelude::*;

struct ExampleModule;

impl Module for ExampleModule {
    fn initialize(&mut self) {
        log::info!("Hello from Rust");
    }
}

implement_unreal_module!(ExampleModule);
