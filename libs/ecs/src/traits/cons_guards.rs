use crate::{
    traits::{
        component_tuple_element::ComponentTupleElement,
        guard::{DynMaybeLockedGuardExt, Guard, MaybeLockedGuard, MaybeLockedGuardExt},
        singleton_tuple_element::SingletonTupleElement,
    },
    world::World,
};

mod private {
    pub trait Sealed {}
}

/// Trait for a tuple of guards
pub trait ConsGuards: private::Sealed + Sized {
    type MaybeLockedGuards: ConsMaybeLockedGuardsExt<LockedGuards = Self>;

    fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards;
}

impl private::Sealed for () {}
impl ConsGuards for () {
    type MaybeLockedGuards = ();
    fn get_maybe_locks(_: &World) -> Self::MaybeLockedGuards {}
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

/// Util trait to convert a cons tuple of components to component sets
pub trait ConsAsComponentSetGuards: private::Sealed {
    type As: ConsGuards;
}

impl ConsAsComponentSetGuards for () {
    type As = ();
}

impl<Head, Tail> ConsAsComponentSetGuards for (Head, Tail)
where
    Head: ComponentTupleElement,
    Tail: ConsAsComponentSetGuards,
{
    type As = (Head::Guard, Tail::As);
}

/// Util trait to convert a cons tuple of components to component sets
pub trait ConsAsSingletonContainerGuards: private::Sealed {
    type As: ConsGuards;
}

impl ConsAsSingletonContainerGuards for () {
    type As = ();
}

impl<Head, Tail> ConsAsSingletonContainerGuards for (Head, Tail)
where
    Head: SingletonTupleElement,
    Tail: ConsAsComponentSetGuards,
{
    type As = (Head::Guard, Tail::As);
}

// Extension trait for a cons tuple of maybe locked guards
pub trait ConsMaybeLockedGuardsExt {
    /// Gets dyn mutable references to the extension trait
    ///
    /// This is needed for erasing the type, sorting, then iterating and locking
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedGuardExt>;

    /// Type of the locked guards that this maybe locked guards
    type LockedGuards;

    /// Converts back from maybe locked to the locked guards
    ///
    /// Note that if any maybe locked guard isn't locked, this WILL panic
    fn to_locked_guards(self) -> Self::LockedGuards;
}

impl ConsMaybeLockedGuardsExt for () {
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedGuardExt> { std::iter::empty() }

    type LockedGuards = ();
    fn to_locked_guards(self) -> Self::LockedGuards {}
}

impl<Head, Tail> ConsMaybeLockedGuardsExt for (Head, Tail)
where
    Head: MaybeLockedGuardExt,
    Tail: ConsMaybeLockedGuardsExt,
{
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedGuardExt> {
        std::iter::once(&mut self.0 as &mut dyn DynMaybeLockedGuardExt).chain(self.1.dyn_muts())
    }

    type LockedGuards = (Head::Guard, Tail::LockedGuards);
    fn to_locked_guards(self) -> Self::LockedGuards { (self.0.to_locked_guard(), self.1.to_locked_guards()) }
}
