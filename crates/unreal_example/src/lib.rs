use unreal::{math::DVec3, prelude::*};

fn setup(mut api: UnrealApi) {
    unreal::log::warn!("//setup");

    let commands = api.spawn_empty_actor(&DVec3::ZERO, &DVec3::ZERO);
    commands.actor().set_label("my_first_actor");

    let commands = api.spawn_empty_actor(&DVec3::ZERO, &DVec3::ZERO);
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
