use std::{any::TypeId, sync::Arc};

use owning_ref::OwningHandle;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{component_set::ComponentSet, traits::component::Component, world::World};

/// Shared read guard for borrowing a component set
pub struct ComponentSetReadGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockReadGuard<'static, ComponentSet<T>>>,
);

/// Shared write guard for borrowing a component set
pub struct ComponentSetWriteGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockWriteGuard<'static, ComponentSet<T>>>,
);

/// Trait for a component set to be created from a world
pub trait ComponentSetGuard: Sized {
    type Component: Component;

    /// Gets the lock from an ecs world for getting this component set guard
    fn get_lock_from_world(world: &World) -> Arc<RwLock<ComponentSet<Self::Component>>>;

    /// Given the appropriate lock, creates the guard
    fn lock(lock: Arc<RwLock<ComponentSet<Self::Component>>>) -> Self;

    /// Locks this component set from the world
    fn lock_from_world(world: &World) -> Self {
        let lock = Self::get_lock_from_world(world);
        Self::lock(lock)
    }
}

impl<T: Component> ComponentSetGuard for ComponentSetReadGuard<T> {
    type Component = T;

    fn get_lock_from_world(world: &World) -> Arc<RwLock<ComponentSet<T>>> { world.component_row_lock::<T>() }
    fn lock(lock: Arc<RwLock<ComponentSet<T>>>) -> Self {
        ComponentSetReadGuard(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.read()))
    }
}

impl<T: Component> ComponentSetGuard for ComponentSetWriteGuard<T> {
    type Component = T;

    fn get_lock_from_world(world: &World) -> Arc<RwLock<ComponentSet<T>>> { world.component_row_lock::<T>() }
    fn lock(lock: Arc<RwLock<ComponentSet<T>>>) -> Self {
        ComponentSetWriteGuard(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.write()))
    }
}

/// Enum to represent if a component set is maybe locked or unlocked
pub enum MaybeLockedComponentSet<G: ComponentSetGuard> {
    Unlocked(Arc<RwLock<ComponentSet<G::Component>>>),
    Lockless,
    Locked(G),
}

/// Extension for maybe locked it can be converted back into a guard
pub trait MaybeLockedComponentSetExt: DynMaybeLockedComponentSetExt {
    type Guard;

    fn to_locked_guard(self) -> Self::Guard;
}

/// Extension for maybe locked so it can be used from a dyn context
pub trait DynMaybeLockedComponentSetExt {
    /// Gets the type id of the component for the component set that is maybe locked
    fn component_type_id(&self) -> TypeId;

    /// If this lock is unlocked, makes this lock locked
    fn lock(&mut self);
}

// Extension trait for a cons tuple of maybe locked guards
pub trait ConsMaybeLockedGuardsExt {
    /// Gets dyn mutable references to the extension trait
    ///
    /// This is needed for erasing the type, sorting, then iterating and locking
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedComponentSetExt>;

    /// Type of the locked guards that this maybe locked guards encapsulates
    type LockedGuards;

    /// Converts back from maybe locked to the locked guards
    ///
    /// Note that if any maybe locked guard isn't locked, this WILL panic
    fn to_locked_guards(self) -> Self::LockedGuards;
}

impl<G: ComponentSetGuard> MaybeLockedComponentSetExt for MaybeLockedComponentSet<G> {
    type Guard = G;

    fn to_locked_guard(self) -> Self::Guard {
        match self {
            MaybeLockedComponentSet::Locked(guard) => guard,
            _ => panic!("Could not convert this to a locked guard"),
        }
    }
}

impl<G: ComponentSetGuard> DynMaybeLockedComponentSetExt for MaybeLockedComponentSet<G> {
    fn component_type_id(&self) -> TypeId { TypeId::of::<G::Component>() }

    fn lock(&mut self) {
        if let MaybeLockedComponentSet::Unlocked(lock) = std::mem::replace(self, MaybeLockedComponentSet::Lockless) {
            *self = MaybeLockedComponentSet::Locked(G::lock(lock))
        }
    }
}

impl ConsMaybeLockedGuardsExt for () {
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedComponentSetExt> { std::iter::empty() }

    type LockedGuards = ();
    fn to_locked_guards(self) -> Self::LockedGuards {}
}

impl<Head, Tail> ConsMaybeLockedGuardsExt for (Head, Tail)
where
    Head: MaybeLockedComponentSetExt,
    Tail: ConsMaybeLockedGuardsExt,
{
    fn dyn_muts(&mut self) -> impl Iterator<Item = &mut dyn DynMaybeLockedComponentSetExt> {
        std::iter::once(&mut self.0 as &mut dyn DynMaybeLockedComponentSetExt).chain(self.1.dyn_muts())
    }

    type LockedGuards = (Head::Guard, Tail::LockedGuards);
    fn to_locked_guards(self) -> Self::LockedGuards { (self.0.to_locked_guard(), self.1.to_locked_guards()) }
}
