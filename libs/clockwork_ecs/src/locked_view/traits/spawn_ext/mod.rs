use crate::{
    entity_id::EntityId,
    locked_view::{LockedView, locked_view_elements::LockedViewElements},
};

mod locked_view_spawn_bundle;

pub use locked_view_spawn_bundle::{LockedViewConsSpawnBundle, LockedViewSpawnBundle};

/// Spawns entities directly from a [`LockedView`](crate::locked_view::LockedView) by
/// inserting bundles of components.
///
/// The `Idxs` type parameter tracks which component sets the bundle touches. In
/// practice, you can pass tuples of components and rely on the blanket bundle
/// implementation.
///
/// # Examples
/// ```rust
/// use clockwork_ecs::locked_view::traits::{LockedViewGetComponentMutExt, LockedViewSpawnExt};
/// use clockwork_ecs::world::World;
///
/// #[derive(Default)]
/// struct Position(f32, f32);
///
/// let world = World::new();
/// let mut view = world.lock_components_view::<(&mut Position,)>();
/// let entity = view.spawn((Position::default(),));
///
/// let mut position = view.get_component_mut::<Position>(entity).unwrap();
/// position.0 = 1.0;
/// ```
pub trait LockedViewSpawnExt<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    /// Creates a new entity and inserts the provided bundle into its component sets.
    fn spawn<B>(&'a mut self, bundle: B) -> EntityId
    where
        B: LockedViewSpawnBundle<'a, C, S, Idxs>;

    // TODO: Batch spawn
}

impl<'a, C, S, Idxs> LockedViewSpawnExt<'a, C, S, Idxs> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn spawn<B>(&'a mut self, bundle: B) -> EntityId
    where
        B: LockedViewSpawnBundle<'a, C, S, Idxs>,
    {
        let entites_arc = self.entities.clone();
        let mut entities = entites_arc.write();
        let id = entities.allocate_id();

        bundle.add_components(id, self);

        id
    }
}
