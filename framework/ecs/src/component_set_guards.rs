use std::sync::Arc;

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
pub trait ComponentSetGuard {
    fn lock_from_world(world: &World) -> Self;
}

impl<T: Component> ComponentSetGuard for ComponentSetReadGuard<T> {
    fn lock_from_world(world: &World) -> Self {
        ComponentSetReadGuard(OwningHandle::new_with_fn(world.component_row_lock::<T>(), |lock| {
            unsafe { &*lock }.read()
        }))
    }
}

impl<T: Component> ComponentSetGuard for ComponentSetWriteGuard<T> {
    fn lock_from_world(world: &World) -> Self {
        ComponentSetWriteGuard(OwningHandle::new_with_fn(world.component_row_lock::<T>(), |lock| {
            unsafe { &*lock }.write()
        }))
    }
}
