use std::sync::Arc;

use owning_ref::OwningHandle;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{component_set::ComponentSet, traits::component::Component, world::World};

pub trait ComponentSetGuard {
    fn from_world(world: &World) -> Self;
}

/// Shared read guard for borrowing a component set
pub struct ComponentSetReadGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockReadGuard<'static, ComponentSet<T>>>,
);

impl<T: Component> ComponentSetGuard for ComponentSetReadGuard<T> {
    fn from_world(world: &World) -> Self {
        ComponentSetReadGuard(OwningHandle::new_with_fn(world.component_row_lock::<T>(), |lock| {
            unsafe { &*lock }.read()
        }))
    }
}

/// Shared write guard for borrowing a component set
pub struct ComponentSetWriteGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockWriteGuard<'static, ComponentSet<T>>>,
);

impl<T: Component> ComponentSetGuard for ComponentSetWriteGuard<T> {
    fn from_world(world: &World) -> Self {
        ComponentSetWriteGuard(OwningHandle::new_with_fn(world.component_row_lock::<T>(), |lock| {
            unsafe { &*lock }.write()
        }))
    }
}
