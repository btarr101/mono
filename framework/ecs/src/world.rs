use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use owning_ref::OwningHandle;
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use static_assertions::assert_impl_all;

use crate::{
    component_set::ComponentSet,
    component_set_guards::{ComponentSetGuard, ComponentSetReadGuard, ComponentSetWriteGuard},
    entity_id_allocator::EntityIdAllocator,
    locked_view::{LockedView, private::LockedViewElements},
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, MutComponentSetMutAccessor},
        singleton::Singleton,
    },
};

assert_impl_all!(World: Send, Sync);

#[derive(Default)]
pub struct World {
    pub entities: Arc<Mutex<EntityIdAllocator>>,
    pub singletons: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
    pub components: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl World {
    /// Creates a new world
    pub fn new() -> Self { Default::default() }

    /// Locks a view of over this world
    ///
    /// Typically, you would use this function at the beginning of a system
    /// so it has guaranteed access to certain sets of components and
    /// singletons.
    pub fn lock_view<Elements: LockedViewElements>(&self) -> LockedView<Elements> { LockedView::new(self) }

    /// Lock a set of components immutably for reading
    pub fn lock_components<T: Component>(&self) -> impl ComponentSetAccessor<T> { ComponentSetReadGuard::from_world(self) }

    /// Lock a set of components mutably for writing
    pub fn lock_components_mut<T: Component>(&self) -> impl MutComponentSetMutAccessor<T> {
        ComponentSetWriteGuard::from_world(self)
    }

    /// Lock a singleton immutably for reading
    pub fn lock_singleton<T: Singleton>(&self) -> OwningHandle<Arc<RwLock<Option<T>>>, RwLockReadGuard<'static, Option<T>>> {
        OwningHandle::new_with_fn(self.singleton_lock::<T>(), |lock| unsafe { &*lock }.read())
    }

    /// Lock a singleton mutably for writing
    pub fn lock_singleton_mut<T: Singleton>(&self) -> OwningHandle<Arc<RwLock<Option<T>>>, RwLockWriteGuard<'static, Option<T>>> {
        OwningHandle::new_with_fn(self.singleton_lock::<T>(), |lock| unsafe { &*lock }.write())
    }

    pub(crate) fn component_row_lock<T: Component>(&self) -> Arc<RwLock<ComponentSet<T>>> {
        let key = TypeId::of::<T>();

        let guard = self.components.read();
        match guard.get(&key) {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.components.write();
                guard
                    .entry(key)
                    .or_insert_with(|| Arc::new(RwLock::new(ComponentSet::<T>::new())))
                    .clone()
            }
        }
        .downcast()
        .expect("downcast")
    }

    fn singleton_lock<T: Singleton>(&self) -> Arc<RwLock<Option<T>>> {
        let key = TypeId::of::<T>();

        let guard = self.singletons.read();
        match guard.get(&key) {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.singletons.write();
                guard.entry(key).or_insert_with(|| Arc::new(RwLock::new(None::<T>))).clone()
            }
        }
        .downcast()
        .expect("downcast")
    }
}
