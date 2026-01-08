use clockwork_tuples::traits::as_cons_tuple::AsConsTuple;

use crate::{
    entity_id::EntityId,
    locked_view::{LockedView, has_components::HasComponentsMut, locked_view_elements::LockedViewElements},
    traits::{component::Component, component_set_accessor::MutComponentSetMutAccessor},
};

/// Describes values that can be inserted as part of [`LockedViewSpawnExt::spawn`](super::LockedViewSpawnExt::spawn).
///
/// Tuples of components automatically implement this trait, but custom bundle
/// structs can implement it manually to perform bespoke initialization.
pub trait LockedViewSpawnBundle<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    /// Inserts the bundle's components for the provided entity into `view`.
    fn add_components(self, id: EntityId, view: &'a mut LockedView<C, S>);
}

impl<'a, C, S, Idxs, Bundle> LockedViewSpawnBundle<'a, C, S, Idxs> for Bundle
where
    C: LockedViewElements,
    S: LockedViewElements,
    Bundle: AsConsTuple,
    Bundle::As: LockedViewConsSpawnBundle<'a, C, S, Idxs>,
{
    fn add_components(self, id: EntityId, view: &'a mut LockedView<C, S>) {
        self.to_cons_tuple().cons_add_components(id, view);
    }
}

/// Internal helper that walks cons-tuples produced by [`AsConsTuple`] to insert
/// each component in order.
pub trait LockedViewConsSpawnBundle<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Self: Sized,
{
    /// Inserts component data represented by the tuple node.
    fn cons_add_components(self, id: EntityId, view: &'a mut LockedView<C, S>);
}

impl<'a, C, S> LockedViewConsSpawnBundle<'a, C, S, ()> for ()
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn cons_add_components(self, _: EntityId, _: &'a mut LockedView<C, S>) {}
}

impl<'a, C, S, Idx, TailIdxs, Head, Tail> LockedViewConsSpawnBundle<'a, C, S, (Idx, TailIdxs)> for (Head, Tail)
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idx: 'static,
    Head: Component,
    Tail: LockedViewConsSpawnBundle<'a, C, S, TailIdxs>,
    LockedView<C, S>: HasComponentsMut<Head, C, Idx>,
{
    fn cons_add_components(self, id: EntityId, view: &'a mut LockedView<C, S>) {
        unsafe { view.get_mut_accessor().add(id, self.0) };
        self.1.cons_add_components(id, view);
    }
}
