use crate::{
    entity_id::EntityId,
    locked_view::{LockedView, locked_view_elements::LockedViewElements},
};

mod locked_view_spawn_bundle;

pub use locked_view_spawn_bundle::{LockedViewConsSpawnBundle, LockedViewSpawnBundle};

pub trait LockedViewSpawnExt<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
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
