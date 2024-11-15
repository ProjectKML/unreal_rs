use unreal::{ecs::prelude::Commands, prelude::*, Startup, Update};

fn setup(mut commands: Commands) {
    unreal::log::warn!("//setup");
}

fn update() {
    unreal::log::warn!("//update");
}

#[derive(Default)]
struct ExampleModule;

impl BuildModule for ExampleModule {
    fn build(&self, module: &mut Module) {
        module
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

implement_module!(ExampleModule);
