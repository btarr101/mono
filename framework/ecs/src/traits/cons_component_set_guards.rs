use crate::{
    component_set_guards::{ComponentSetGuard, ConsMaybeLockedGuardsExt, MaybeLockedComponentSet},
    traits::component_tuple_element::ComponentTupleElement,
    world::World,
};

mod sealed {
    pub trait Sealed {}
}

/// Trait for a tuple set of component sets
pub trait ConsComponentSetGuards: sealed::Sealed + Sized {
    type MaybeLockedGuards: ConsMaybeLockedGuardsExt<LockedGuards = Self>;
    fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards;
}
/// Util trait to convert a cons tuple of components to component sets
pub trait ConsAsComponentSetGuards: sealed::Sealed {
    type As: ConsComponentSetGuards;
}

impl sealed::Sealed for () {}
impl ConsComponentSetGuards for () {
    type MaybeLockedGuards = ();
    fn get_maybe_locks(_: &World) -> Self::MaybeLockedGuards {}
}
impl ConsAsComponentSetGuards for () {
    type As = ();
}

impl<Head, Tail> sealed::Sealed for (Head, Tail) {}
impl<Head, Tail> ConsComponentSetGuards for (Head, Tail)
where
    Head: ComponentSetGuard,
    Tail: ConsComponentSetGuards,
{
    type MaybeLockedGuards = (MaybeLockedComponentSet<Head>, Tail::MaybeLockedGuards);
    fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards {
        (
            MaybeLockedComponentSet::Unlocked(Head::get_lock_from_world(world)),
            Tail::get_maybe_locks(world),
        )
    }
}
impl<Head, Tail> ConsAsComponentSetGuards for (Head, Tail)
where
    Head: ComponentTupleElement,
    Tail: ConsAsComponentSetGuards,
{
    type As = (Head::Guard, Tail::As);
}
