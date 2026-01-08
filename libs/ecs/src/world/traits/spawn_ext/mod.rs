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
/// ```no_run
/// use ecs::world::{traits::spawn_ext::SpawnExt, World};
///
/// #[derive(Default)]
/// struct Position(f32, f32);
///
/// let world = World::new();
/// let entity = <World as SpawnExt<'_, (&mut Position,)>>::spawn(&world, (Position::default(),));
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
