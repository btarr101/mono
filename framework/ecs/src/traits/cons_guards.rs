use crate::{
    traits::{
        component_tuple_element::ComponentTupleElement,
        guard::{ConsMaybeLockedGuardsExt, Guard, MaybeLockedGuard},
    },
    world::World,
};

mod private {
    pub trait Sealed {}
}

/// Trait for a tuple set of component sets
pub trait ConsGuards: private::Sealed + Sized {
    type MaybeLockedGuards: ConsMaybeLockedGuardsExt<LockedGuards = Self>;

    fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards;
}
/// Util trait to convert a cons tuple of components to component sets
pub trait ConsAsGuards: private::Sealed {
    type As: ConsGuards;
}

impl private::Sealed for () {}
impl ConsGuards for () {
    type MaybeLockedGuards = ();
    fn get_maybe_locks(_: &World) -> Self::MaybeLockedGuards {}
}
impl ConsAsGuards for () {
    type As = ();
}

impl<Head, Tail> private::Sealed for (Head, Tail) {}
impl<Head, Tail> ConsGuards for (Head, Tail)
where
    Head: Guard,
    Tail: ConsGuards,
{
    type MaybeLockedGuards = (MaybeLockedGuard<Head>, Tail::MaybeLockedGuards);
    fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards {
        (
            MaybeLockedGuard::Unlocked(Head::get_lock_from_world(world)),
            Tail::get_maybe_locks(world),
        )
    }
}
impl<Head, Tail> ConsAsGuards for (Head, Tail)
where
    Head: ComponentTupleElement,
    Tail: ConsAsGuards,
{
    type As = (Head::Guard, Tail::As);
}
