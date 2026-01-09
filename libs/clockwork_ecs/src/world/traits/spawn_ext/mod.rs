use crate::{
    entity_id::EntityId,
    world::{World, traits::spawn_ext::world_spawn_bundle::SpawnBundle},
};

mod world_spawn_bundle;

/// Allows spawning entities directly through [`World`] without manually locking views.
///
/// The `Idxs` type argument tracks which component sets the bundle will touch.
/// In practice you select it through fully qualified syntax, as shown below.
///
/// # Examples
/// ```rust
/// use clockwork_ecs::world::{traits::spawn_ext::SpawnExt, World};
///
/// let world = World::new();
/// let entity = world.spawn((0i32,));
/// assert!(world.entity_exists(entity));
/// ```
pub trait SpawnExt<'a, Idxs> {
    /// Spawns an entity by delegating to the provided bundle implementation.
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
