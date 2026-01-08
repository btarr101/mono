use clockwork_tuples::traits::{as_cons_tuple::AsConsTuple, as_ref_tuple::AsRefTuple};

use crate::{
    entity_id::EntityId,
    locked_view::{
        LockedView,
        traits::{LockedViewSpawnBundle, LockedViewSpawnExt},
    },
    traits::cons_guards::{ConsAsComponentSetGuards, ConsAsSingletonContainerGuards},
    world::World,
};

pub trait SpawnBundle<'a, Idxs> {
    fn spawn(self, world: &'a World) -> EntityId;
}

impl<'a, Idxs, Bundle> SpawnBundle<'a, Idxs> for Bundle
where
    Self: 'a + Sized,
    Self: AsRefTuple,
    <Self as AsRefTuple>::AsMuts<'a>: AsConsTuple,
    <<Self as AsRefTuple>::AsMuts<'a> as AsConsTuple>::As: ConsAsComponentSetGuards,
    <<Self as AsRefTuple>::AsMuts<'a> as AsConsTuple>::As: ConsAsSingletonContainerGuards,
    for<'b> Self: LockedViewSpawnBundle<'b, <Self as AsRefTuple>::AsMuts<'a>, (), Idxs>,
{
    fn spawn(self, world: &'a World) -> EntityId { LockedView::<<Self as AsRefTuple>::AsMuts<'a>, ()>::new(world).spawn(self) }
}
