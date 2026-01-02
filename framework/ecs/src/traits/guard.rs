use std::{any::TypeId, sync::Arc};

use owning_ref::OwningHandle;
use parking_lot::RwLock;

use crate::{
    component_set::ComponentSet,
    component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    singleton_guards::{OptionalSingletonContainerReadGuard, OptionalSingletonContainerWriteGuard},
    traits::{component::Component, singleton::Singleton},
    world::World,
};

pub trait Guard: Sized {
    type Lock;
    type Element: 'static;

    /// Gets the lock from an ecs world for getting this component set guard
    fn get_lock_from_world(world: &World) -> Self::Lock;

    /// Given the appropriate lock, creates the guard
    fn lock(lock: Self::Lock) -> Self;

    /// Locks from the world
    fn lock_from_world(world: &World) -> Self {
        let lock = Self::get_lock_from_world(world);
        Self::lock(lock)
    }
}

/// Enum to represent if gaurd exists or not
pub enum MaybeLockedGuard<G: Guard> {
    Unlocked(G::Lock),
    Lockless,
    Locked(G),
}

/// Extension for maybe locked so it can be used from a dyn context
pub trait DynMaybeLockedGuardExt {
    /// Gets the type id of the container that is being guarded
    fn element_type_id(&self) -> TypeId;

    /// If this lock is unlocked, makes this lock locked
    fn lock(&mut self);
}

/// Extension for maybe locked it can be converted back into a guard
pub trait MaybeLockedGuardExt: DynMaybeLockedGuardExt {
    type Guard;

    fn to_locked_guard(self) -> Self::Guard;
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

impl<T: Component> Guard for ComponentSetReadGuard<T> {
    type Lock = Arc<RwLock<ComponentSet<T>>>;
    type Element = T;

    fn get_lock_from_world(world: &World) -> Self::Lock { world.component_set_lock::<T>() }
    fn lock(lock: Arc<RwLock<ComponentSet<T>>>) -> Self {
        ComponentSetReadGuard(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.read()))
    }
}

impl<T: Component> Guard for ComponentSetWriteGuard<T> {
    type Lock = Arc<RwLock<ComponentSet<T>>>;
    type Element = T;

    fn get_lock_from_world(world: &World) -> Self::Lock { world.component_set_lock() }
    fn lock(lock: Self::Lock) -> Self { Self(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.write())) }
}

impl<T: Singleton> Guard for OptionalSingletonContainerReadGuard<T> {
    type Lock = Arc<RwLock<Option<T>>>;
    type Element = T;

    fn get_lock_from_world(world: &World) -> Self::Lock { world.singleton_lock() }
    fn lock(lock: Self::Lock) -> Self { Self::from_lock(lock) }
}

impl<T: Singleton> Guard for OptionalSingletonContainerWriteGuard<T> {
    type Lock = Arc<RwLock<Option<T>>>;
    type Element = T;

    fn get_lock_from_world(world: &World) -> Self::Lock { world.singleton_lock() }
    fn lock(lock: Self::Lock) -> Self { Self::from_lock(lock) }
}

impl<G: Guard> DynMaybeLockedGuardExt for MaybeLockedGuard<G> {
    fn element_type_id(&self) -> TypeId { TypeId::of::<G::Element>() }

    fn lock(&mut self) {
        if let MaybeLockedGuard::Unlocked(lock) = std::mem::replace(self, MaybeLockedGuard::Lockless) {
            *self = MaybeLockedGuard::Locked(G::lock(lock))
        }
    }
}

impl<G: Guard> MaybeLockedGuardExt for MaybeLockedGuard<G> {
    type Guard = G;

    fn to_locked_guard(self) -> Self::Guard {
        match self {
            MaybeLockedGuard::Locked(guard) => guard,
            _ => panic!("Could not convert this to a locked guard"),
        }
    }
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
