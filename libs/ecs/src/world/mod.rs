//! World state and storage for entities, components, and singletons.
//!
//! This module defines the `World` type, which owns entity identifiers and
//! provides synchronized access to component sets and singleton values.
//! All mutation is coordinated through locking to uphold aliasing and
//! borrowing invariants across systems.

use std::{any::Any, sync::Arc};

use parking_lot::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use static_assertions::assert_impl_all;

use crate::{
    entity_id::EntityId,
    locked_view::{LockedView, locked_view_elements::LockedViewElements},
    traits::{component::Component, component_set_accessor::MutComponentSetMutAccessor, guard::Guard, singleton::Singleton},
    util::{defered_queue::RotatingLockedDeferedQueue, sorted_type_arcmap::SortedTypeArcMap},
    world::{
        component_set::{AnyComponentSet, ComponentSet, component_set_guards::ComponentSetWriteGuard},
        entity_id_allocator::EntityIdAllocator,
        singleton_container::{
            SingletonContainer,
            singleton_guards::{SingletonContainerEntry, SingletonContainerReadGuard, SingletonContainerWriteGuard},
        },
    },
};

pub(crate) mod component_set;
pub(crate) mod entity_id_allocator;
pub(crate) mod singleton_container;
pub mod traits;

assert_impl_all!(World: Send, Sync);

/// Central ECS storage for entities, components, and singletons.
///
/// The `World` type owns all entity identifiers plus the component and
/// singleton containers that back [`LockedView`](crate::locked_view::LockedView)
/// operations.
///
/// # Examples
/// ```no_run
/// use ecs::locked_view::traits::{LockedViewGetComponentMutExt, LockedViewSpawnExt};
/// use ecs::world::World;
///
/// #[derive(Default)]
/// struct Position(f32, f32);
///
/// let world = World::new();
/// let mut view = world.lock_components_view::<(&mut Position,)>();
/// let entity = view.spawn((Position::default(),));
/// let mut position = view.get_component_mut::<Position>(entity).unwrap();
/// position.0 = 1.0;
/// assert!(world.entity_exists(entity));
/// ```
#[derive(Default)]
pub struct World {
    pub(crate) entities: Arc<RwLock<EntityIdAllocator>>,
    pub(crate) singletons: RwLock<SortedTypeArcMap<dyn Any + Send + Sync>>,
    pub(crate) components: RwLock<SortedTypeArcMap<dyn AnyComponentSetRwLock>>,
    pub(crate) defered_updates: Arc<RotatingLockedDeferedQueue<World>>,
}

impl World {
    /// Creates an empty world with no entities or storage initialized.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Allocates a fresh [`EntityId`] without touching component storage.
    ///
    /// The returned identifier can subsequently be populated via locked views or
    /// direct component-set operations.
    pub fn create_entity(&self) -> EntityId {
        self.entities.write().allocate_id()
    }

    /// Returns `true` when `id` still refers to a live entity.
    pub fn entity_exists(&self, id: EntityId) -> bool {
        self.entities.read().index_in_use(id.index)
    }

    /// Adds `component` to `id`, locking the backing component set for writing.
    ///
    /// Returns `true` when the component was inserted instead of replaced.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::world::World;
    ///
    /// #[derive(Default)]
    /// struct Position(f32, f32);
    ///
    /// let world = World::new();
    /// let entity = world.create_entity();
    /// assert!(world.require_components_and_add(entity, Position::default()));
    /// ```
    pub fn require_components_and_add<T: Component>(&self, id: EntityId, component: T) -> bool {
        unsafe { ComponentSetWriteGuard::lock_from_world(self).try_add(id, component).is_some() }
    }

    /// Removes `T` from `id`, returning the removed component when present.
    ///
    /// This acquires the component set's write lock, so prefer deferred removal
    /// from a [`LockedView`](crate::locked_view::LockedView) when you already
    /// hold access to the same set.
    pub fn require_components_and_pop<T: Component>(&self, id: EntityId) -> Option<T> {
        ComponentSetWriteGuard::lock_from_world(self).soft_pop(id)
    }

    /// Destroys an entity plus every component stored for it.
    ///
    /// This acquires write access to every registered component set, so callers
    /// must ensure no other outstanding locks exist to avoid deadlock.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::world::World;
    ///
    /// let world = World::new();
    /// let entity = world.create_entity();
    /// world.require_all_components_and_destroy_entity(entity);
    /// assert!(!world.entity_exists(entity));
    /// ```
    pub fn require_all_components_and_destroy_entity(&self, id: EntityId) {
        // First, clear out components. If we fail to use an old entity id boo hoo. If we reallocate an entity id
        // with already existing components we are in trouble
        {
            // Grab all locks and collect first, that way guards have something to reference
            // (if we were less lazy we could bundle with OwningRef)
            let locks = self.components.read().values().cloned().collect::<Vec<_>>();

            // Ensure we lock every single component set before moving to actually delete the components, that way this operation is kept
            // atomic
            //
            // Note this is safe and won;t cause deadlocks because iterating our backing store is in sorted order
            let component_sets = locks.iter().map(|lock| lock.write()).collect::<Vec<_>>();
            component_sets
                .into_iter()
                .for_each(|mut component_set| component_set.remove(id.index));
        }

        self.entities.write().free_id(id);
    }

    /// Attempts to immutably borrow the singleton of type `T`.
    ///
    /// Returns `None` if the singleton is not currently stored.
    pub fn lock_singleton<T: Singleton>(&self) -> Option<SingletonContainerReadGuard<T>> {
        SingletonContainerReadGuard::try_from_lock(self.singleton_container_lock())
    }

    /// Attempts to mutably borrow the singleton of type `T`.
    pub fn lock_singleton_mut<T: Singleton>(&self) -> Option<SingletonContainerWriteGuard<T>> {
        SingletonContainerWriteGuard::try_from_lock(self.singleton_container_lock())
    }

    /// Provides entry-style access for inserting or updating a singleton in-place.
    pub fn lock_singleton_entry<T: Singleton>(&self) -> SingletonContainerEntry<T> {
        SingletonContainerEntry::from_lock(self.singleton_container_lock())
    }

    /// Locks a view over this world with pre-declared component and singleton access.
    ///
    /// Provide tuples of `&T`/`&mut T` specifiers to describe which data should
    /// be locked.
    pub fn lock_view<C: LockedViewElements, S: LockedViewElements>(&self) -> LockedView<C, S> {
        LockedView::new(self)
    }

    /// Locks a view scoped to components only.
    pub fn lock_components_view<C: LockedViewElements>(&self) -> LockedView<C, ()> {
        LockedView::new(self)
    }

    /// Locks a view scoped to singletons only.
    pub fn lock_singletons_view<S: LockedViewElements>(&self) -> LockedView<(), S> {
        LockedView::new(self)
    }

    /// Flushes all deferred operations queued by locked views.
    ///
    /// This must be called when no component or singleton locks are held.
    pub fn require_all_and_execute_defered_updates(&self) {
        self.defered_updates.pop_all(self);
    }

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
    pub(crate) fn singleton_container_lock<T: Singleton>(&self) -> Arc<RwLock<SingletonContainer<T>>> {
        let guard = self.singletons.read();
        match guard.get::<T>() {
            Some(arc) => arc.clone(),
            None => {
                drop(guard);

                let mut guard = self.singletons.write();
                guard
                    .entry::<T>()
                    .or_insert_with(|| Arc::new(RwLock::new(SingletonContainer::<T>::new())))
                    .clone()
            }
        }
        .downcast()
        .expect("downcast")
    }
}

/// Trait that type-erases component set locks for heterogeneous storage.
pub(crate) trait AnyComponentSetRwLock: Send + Sync {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet>;
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: AnyComponentSet> AnyComponentSetRwLock for RwLock<T> {
    fn write(&self) -> MappedRwLockWriteGuard<'_, dyn AnyComponentSet> {
        RwLockWriteGuard::map(self.write(), |t| t as &mut dyn AnyComponentSet)
    }
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}
