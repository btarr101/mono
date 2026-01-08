use crate::{
    entity_id::EntityId,
    world::{World, traits::spawn_ext::world_spawn_bundle::SpawnBundle},
};

mod world_spawn_bundle;

pub trait SpawnExt<'a, Idxs> {
    fn spawn<B>(&'a self, bundle: B) -> EntityId
    where
        B: SpawnBundle<'a, Idxs>;
}

impl<'a, Idxs> SpawnExt<'a, Idxs> for World {
    fn spawn<B>(&'a self, bundle: B) -> EntityId
    where
        B: SpawnBundle<'a, Idxs>,
    {
        bundle.spawn(self)
    }
}
