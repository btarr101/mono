use std::{any::Any, sync::Arc};

use owning_ref::OwningHandle;
use parking_lot::{MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use static_assertions::assert_impl_all;

use crate::{
    component_set::{AnyComponentSet, ComponentSet},
    entity::{Entity, EntityId},
    entity_id_allocator::EntityIdAllocator,
    locked_view::{LockedView, private::LockedViewElements},
    sorted_hasmap::SortedTypeArcMap,
    traits::{component::Component, singleton::Singleton},
};

assert_impl_all!(World: Send, Sync);

#[derive(Default)]
pub struct World {
    pub entities: Arc<RwLock<EntityIdAllocator>>,
    pub singletons: RwLock<SortedTypeArcMap<dyn Any + Send + Sync>>,
    pub components: RwLock<SortedTypeArcMap<dyn AnyComponentSetRwLock>>,
}

impl World {
    /// Creates a new world
    pub fn new() -> Self { Default::default() }

    /// Creates a new entity in this world
    pub fn create_entity(&self) -> Entity<'_> {
        let id = { self.entities.write().allocate_id() };
        Entity::new(id, self)
    }

    /// Gets an entity from this world
    pub fn get_entity(&self, id: EntityId) -> Option<Entity<'_>> {
        self.entities.read().index_in_use(id.index).then_some(Entity::new(id, self))
    }

    /// Locks a view of over this world
    ///
    /// Typically, you would use this function at the beginning of a system
    /// so it has guaranteed access to certain sets of components and
    /// singletons.
    pub fn lock_view<Elements: LockedViewElements>(&self) -> LockedView<Elements> { LockedView::new(self) }

    /// Lock a singleton immutably for reading
    pub fn lock_singleton<T: Singleton>(&self) -> OwningHandle<Arc<RwLock<Option<T>>>, RwLockReadGuard<'static, Option<T>>> {
        OwningHandle::new_with_fn(self.singleton_lock::<T>(), |lock| unsafe { &*lock }.read())
    }

    /// Lock a singleton mutably for writing
    pub fn lock_singleton_mut<T: Singleton>(&self) -> OwningHandle<Arc<RwLock<Option<T>>>, RwLockWriteGuard<'static, Option<T>>> {
        OwningHandle::new_with_fn(self.singleton_lock::<T>(), |lock| unsafe { &*lock }.write())
    }

    pub(crate) fn component_row_lock<T: Component>(&self) -> Arc<RwLock<ComponentSet<T>>> {
        let guard = self.components.read();
        match guard.get::<T>() {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.components.write();
                guard
                    .entry::<T>()
                    .or_insert_with(|| Arc::new(RwLock::new(ComponentSet::<T>::new())))
                    .clone()
            }
        }
        .as_any()
        .downcast()
        .expect("downcast")
    }

    fn singleton_lock<T: Singleton>(&self) -> Arc<RwLock<Option<T>>> {
        let guard = self.singletons.read();
        match guard.get::<T>() {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.singletons.write();
                guard.entry::<T>().or_insert_with(|| Arc::new(RwLock::new(None::<T>))).clone()
            }
        }
        .downcast()
        .expect("downcast")
    }
}

/// Util trait to allow removing a component from a rwlocked component set,
/// and from converting that set to any for downcasting
pub trait AnyComponentSetRwLock: Send + Sync {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet>;
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: AnyComponentSet> AnyComponentSetRwLock for RwLock<T> {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet> {
        RwLockWriteGuard::map(self.write(), |t| t as &mut dyn AnyComponentSet)
    }
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> { self }
}
