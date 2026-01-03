use smallvec::SmallVec;
use tuples::traits::as_cons_tuple::AsConsTuple;

use crate::{
    traits::cons_guards::{ConsAsComponentSetGuards, ConsAsSingletonContainerGuards, ConsGuards, ConsMaybeLockedGuardsExt},
    world::World,
};

/// C that are used to identify a locked view.
pub trait LockedViewElements {
    type ComponentSetGuards: ConsGuards;
    type SingletonContainerGuards: ConsGuards;

    /// Gets a cons style tuple of all the guards of component sets in the world
    fn lock_component_sets_from_world(world: &World) -> Self::ComponentSetGuards;

    /// Gets a cons style tuple of all the guards of singleton containers in the world
    fn lock_singleton_containers_from_world(world: &World) -> Self::SingletonContainerGuards;
}

impl<T> LockedViewElements for T
where
    Self: AsConsTuple,
    <Self as AsConsTuple>::As: ConsAsComponentSetGuards,
    <Self as AsConsTuple>::As: ConsAsSingletonContainerGuards,
{
    type ComponentSetGuards = <<Self as AsConsTuple>::As as ConsAsComponentSetGuards>::As;
    type SingletonContainerGuards = <<Self as AsConsTuple>::As as ConsAsSingletonContainerGuards>::As;

    fn lock_component_sets_from_world(world: &World) -> Self::ComponentSetGuards {
        let mut maybe_locks = Self::ComponentSetGuards::get_maybe_locks(world);
        lock_maybe_locks(&mut maybe_locks);
        maybe_locks.to_locked_guards()
    }

    fn lock_singleton_containers_from_world(world: &World) -> Self::SingletonContainerGuards {
        let mut maybe_locks = Self::SingletonContainerGuards::get_maybe_locks(world);
        lock_maybe_locks(&mut maybe_locks);
        maybe_locks.to_locked_guards()
    }
}

fn lock_maybe_locks<T: ConsMaybeLockedGuardsExt>(maybe_locks: &mut T) {
    // Collect dyn references to all of the maybe locks in a Vec (todo: use smallvec to keep this on the stack)
    let mut dyn_maybe_locks = maybe_locks.dyn_muts().collect::<SmallVec<[_; 8]>>();

    // !! Important !!
    // We sort the dyn locks by the type id of the component. That way we have a stable lock order to prevent deadlocks,
    // then lock them
    dyn_maybe_locks.sort_by_key(|dyn_lock| dyn_lock.element_type_id());
    dyn_maybe_locks.into_iter().for_each(|dyn_lock| dyn_lock.lock());
}
