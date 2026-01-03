use std::sync::Arc;

use owning_ref::OwningHandle;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{traits::component::Component, world::component_set::ComponentSet};

/// Shared read guard for borrowing a component set
pub struct ComponentSetReadGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockReadGuard<'static, ComponentSet<T>>>,
);

/// Shared write guard for borrowing a component set
pub struct ComponentSetWriteGuard<T: Component>(
    pub(crate) OwningHandle<Arc<RwLock<ComponentSet<T>>>, RwLockWriteGuard<'static, ComponentSet<T>>>,
);
