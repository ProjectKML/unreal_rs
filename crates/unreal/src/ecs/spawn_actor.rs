use std::ptr;

use crate::{
    bindings,
    ecs::{prelude::*, UWorld},
    Actor,
};

pub trait SpawnActorCommands {
    fn spawn_empty_actor(
        &mut self,
        callback: impl FnOnce(&mut Actor) + Send + Sync + 'static,
    ) -> EntityCommands;
    fn spawn_actor<T: Bundle>(
        &mut self,
        bundle: T,
        callback: impl FnOnce(&mut Actor) + Send + Sync + 'static,
    ) -> EntityCommands;
}

impl SpawnActorCommands for Commands<'_, '_> {
    fn spawn_empty_actor(
        &mut self,
        callback: impl FnOnce(&mut Actor) + Send + Sync + 'static,
    ) -> EntityCommands {
        let mut entity = self.spawn_empty();

        entity.queue(move |entity: Entity, mut world: &mut World| {
            let uworld = world.resource_mut::<UWorld>().0;

            let mut actor = Actor(unsafe {
                (bindings::get().UWorld_SpawnECSActor)(
                    uworld,
                    entity.to_bits(),
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                )
            });

            callback(&mut actor);
        });

        entity
    }

    fn spawn_actor<T: Bundle>(
        &mut self,
        bundle: T,
        callback: impl FnOnce(&mut Actor) + Send + Sync + 'static,
    ) -> EntityCommands {
        let mut entity = self.spawn_empty();
        entity.insert(bundle);

        entity.queue(move |entity: Entity, mut world: &mut World| {
            let uworld = world.resource_mut::<UWorld>().0;

            let mut actor = Actor(unsafe {
                (bindings::get().UWorld_SpawnECSActor)(
                    uworld,
                    entity.to_bits(),
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                )
            });

            callback(&mut actor);
        });

        entity
    }
}
