use crate::{
    entity::LockedViewEntity,
    locked_view::{
        LockedView, locked_view_elements::LockedViewElements, traits::spawn_ext::locked_view_spawn_bundle::LockedViewSpawnBundle,
    },
};

mod locked_view_spawn_bundle;

pub trait LockedViewSpawnExt<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn spawn<B>(&'a mut self, bundle: B) -> LockedViewEntity<'a, &'a mut LockedView<C, S>>
    where
        B: LockedViewSpawnBundle<'a, C, S, Idxs>;
}

impl<'a, C, S, Idxs> LockedViewSpawnExt<'a, C, S, Idxs> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn spawn<B>(&'a mut self, bundle: B) -> LockedViewEntity<'a, &'a mut LockedView<C, S>>
    where
        B: LockedViewSpawnBundle<'a, C, S, Idxs>,
    {
        bundle.spawn(self)
    }
}
