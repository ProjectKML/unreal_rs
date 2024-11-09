use unreal::prelude::*;

#[derive(Default)]
struct ExampleModule;

impl BuildModule for ExampleModule {
    fn build(&self, module: &mut Module) {
        module.insert_resource().add_systems()
    }
}

implement_module!(ExampleModule);
