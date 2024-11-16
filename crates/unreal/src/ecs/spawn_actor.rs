use crate::{
    bindings,
    ecs::{prelude::*, UWorld},
    ffi,
};

pub trait SpawnActorCommands {
    fn spawn_empty_actor(&mut self, name: impl Into<String>) -> EntityCommands;
    fn spawn_actor<T: Bundle>(&mut self, name: impl Into<String>, bundle: T) -> EntityCommands;
}

impl SpawnActorCommands for Commands<'_, '_> {
    fn spawn_empty_actor(&mut self, name: impl Into<String>) -> EntityCommands {
        let name = name.into();
        let mut entity = self.spawn_empty();

        entity.queue(move |entity: Entity, mut world: &mut World| {
            let uworld = world.resource_mut::<UWorld>().0;

            let spawn_parameters = ffi::FActorSpawnParameters {
                NamePtr: name.as_ptr().cast(),
                NameLen: name.len(),
                Template: std::ptr::null_mut(),
                Owner: std::ptr::null_mut(),
            };

            unsafe {
                (bindings::get().UWorld_SpawnECSActor)(
                    uworld,
                    entity.to_bits(),
                    std::ptr::null(),
                    std::ptr::null(),
                    &spawn_parameters,
                );
            }
        });

        entity
    }

    fn spawn_actor<T: Bundle>(&mut self, name: impl Into<String>, bundle: T) -> EntityCommands {
        todo!()
    }
}
