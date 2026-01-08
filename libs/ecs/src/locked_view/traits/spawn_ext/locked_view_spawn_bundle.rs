use clockwork_tuples::traits::as_cons_tuple::AsConsTuple;

use crate::{
    entity::{LockedViewEntity, LockedViewEntityComponentMutExt},
    locked_view::{LockedView, has_components::HasComponentsMut, locked_view_elements::LockedViewElements},
    traits::component::Component,
};

pub trait LockedViewSpawnBundle<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn spawn(self, view: &'a mut LockedView<C, S>) -> LockedViewEntity<'a, &'a mut LockedView<C, S>>;
}

impl<'a, C, S, Idxs, Bundle> LockedViewSpawnBundle<'a, C, S, Idxs> for Bundle
where
    C: LockedViewElements,
    S: LockedViewElements,
    Bundle: AsConsTuple,
    Bundle::As: LockedViewConsSpawnBundle<'a, C, S, Idxs>,
{
    fn spawn(self, view: &'a mut LockedView<C, S>) -> LockedViewEntity<'a, &'a mut LockedView<C, S>> {
        self.to_cons_tuple().cons_spawn(view)
    }
}

pub trait LockedViewConsSpawnBundle<'a, C, S, Idxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Self: Sized,
{
    fn cons_spawn(self, view: &'a mut LockedView<C, S>) -> LockedViewEntity<'a, &'a mut LockedView<C, S>> {
        let mut entity = view.create_entity();

        self.cons_add_components(&mut entity);

        entity
    }

    fn cons_add_components(self, entity: &mut LockedViewEntity<'_, &mut LockedView<C, S>>);
}

impl<'a, C, S> LockedViewConsSpawnBundle<'a, C, S, ()> for ()
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn cons_add_components(self, _: &mut LockedViewEntity<'_, &mut LockedView<C, S>>) {}
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
    fn cons_add_components(self, entity: &mut LockedViewEntity<'_, &mut LockedView<C, S>>) {
        entity.add(self.0);
        self.1.cons_add_components(entity);
    }
}
