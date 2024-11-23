use crate::{
    actor::{ActorSpawnCallback, ActorSpawnParams},
    ecs::bundle::Bundle,
    hierarchy::ChildBuild,
};

pub trait ActorChildBuild: ChildBuild {
    fn spawn_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
        bundle: impl Bundle,
    ) -> Self::SpawnOutput<'_>;

    fn spawn_empty(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
    ) -> Self::SpawnOutput<'_>;
}

impl<T: ChildBuild> ActorChildBuild for T {
    fn spawn_actor(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
        bundle: impl Bundle,
    ) -> Self::SpawnOutput<'_> {
        let output = self.spawn(bundle);

        //TODO:

        output
    }

    fn spawn_empty(
        &mut self,
        params: ActorSpawnParams,
        callback: impl ActorSpawnCallback,
    ) -> Self::SpawnOutput<'_> {
        let output = self.spawn_empty();

        //TODO:

        output
    }
}
