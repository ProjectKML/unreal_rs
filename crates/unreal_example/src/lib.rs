use unreal::{actor::ActorSpawnParams, ecs::ActorCommands, object::Subclass, prelude::*};

fn setup(mut commands: Commands) {
    unreal::log::warn!("//setup");

    commands.spawn_empty_actor(ActorSpawnParams::default(), |actor| {
        actor.set_label("my_first_rust_actor");
    });

    commands.spawn_empty_actor(ActorSpawnParams::default(), |actor| {
        actor.set_label("my_second_rust_actor");
    });
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
