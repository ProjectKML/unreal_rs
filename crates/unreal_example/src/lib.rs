use unreal::prelude::*;

#[derive(Default)]
struct ExampleModule;

impl BuildModule for ExampleModule {
    fn build(&self, module: &mut Module) {
        unreal::log::warn!("Hello, World! from Rust");
    }
}

implement_module!(ExampleModule);
