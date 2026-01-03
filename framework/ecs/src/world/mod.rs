use std::{any::Any, sync::Arc};

use parking_lot::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use static_assertions::assert_impl_all;

use crate::{
    entity::{Entity, EntityId},
    locked_view::{LockedView, locked_view_elements::LockedViewElements},
    traits::{component::Component, singleton::Singleton},
    util::sorted_type_arcmap::SortedTypeArcMap,
    world::{
        component_set::{AnyComponentSet, ComponentSet},
        entity_id_allocator::EntityIdAllocator,
        singleton_guards::{SingletonContainerEntry, SingletonContainerReadGuard, SingletonContainerWriteGuard},
    },
};

pub(crate) mod component_set;
pub(crate) mod entity_id_allocator;
pub(crate) mod singleton_guards;

assert_impl_all!(World: Send, Sync);

#[derive(Default)]
pub struct World {
    pub(crate) entities: Arc<RwLock<EntityIdAllocator>>,
    pub(crate) singletons: RwLock<SortedTypeArcMap<dyn Any + Send + Sync>>,
    pub(crate) components: RwLock<SortedTypeArcMap<dyn AnyComponentSetRwLock>>,
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

    /// Lock a singleton immutably for reading
    pub fn lock_singleton<T: Singleton>(&self) -> Option<SingletonContainerReadGuard<T>> {
        SingletonContainerReadGuard::try_from_lock(self.singleton_lock())
    }

    /// Locks a singleton mutably for writing
    pub fn lock_singleton_mut<T: Singleton>(&self) -> Option<SingletonContainerWriteGuard<T>> {
        SingletonContainerWriteGuard::try_from_lock(self.singleton_lock())
    }

    /// Locks an entry to a singleton
    pub fn lock_singleton_entry<T: Singleton>(&self) -> SingletonContainerEntry<T> {
        SingletonContainerEntry::from_lock(self.singleton_lock())
    }

    /// Locks a view of over this world
    ///
    /// Typically, you would use this function at the beginning of a system
    /// so it has guaranteed access to certain sets of components and
    /// singletons.
    pub fn lock_view<C: LockedViewElements, S: LockedViewElements>(&self) -> LockedView<C, S> { LockedView::new(self) }

    /// Gets the lock to a particular component set
    pub(crate) fn component_set_lock<T: Component>(&self) -> Arc<RwLock<ComponentSet<T>>> {
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

    /// Gets the lock to a singleton container
    pub(crate) fn singleton_lock<T: Singleton>(&self) -> Arc<RwLock<Option<T>>> {
        let guard = self.singletons.read();
        match guard.get::<T>() {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.singletons.write();
                guard
                    .entry::<T>()
                    .or_insert_with(|| Arc::new(RwLock::new(Option::<T>::None)))
                    .clone()
            }
        }
        .downcast()
        .expect("downcast")
    }
}

/// Util trait to allow removing a component from a rwlocked component set,
/// and from converting that set to any for downcasting
pub(crate) trait AnyComponentSetRwLock: Send + Sync {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet>;
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: AnyComponentSet> AnyComponentSetRwLock for RwLock<T> {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet> {
        RwLockWriteGuard::map(self.write(), |t| t as &mut dyn AnyComponentSet)
    }
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> { self }
}
