//! Entities and entity-scoped access APIs.
//!
//! This module defines entity identifiers, entity handles, and extensions
//! for accessing components through locked views.

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    locked_view::{
        LockedView,
        has_components::{HasComponents, HasComponentsMut},
        locked_view_elements::LockedViewElements,
    },
    traits::{component::Component, component_set_accessor::MutComponentSetMutAccessor, guard::Guard},
    world::{World, component_set::component_set_guards::ComponentSetWriteGuard},
};

/// Stable identifier for an entity and its components within a `World`.
#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

/// Handle for interacting with an entity inside a `World`.
///
/// Instances are short-lived and borrow the world so that component access is
/// routed through locking APIs.
pub struct Entity<'a> {
    id: EntityId,
    world: &'a World,
}

impl<'a> Entity<'a> {
    pub(crate) fn new(id: EntityId, world: &'a World) -> Self { Self { id, world } }

    /// Adds a component to this enity
    ///
    /// Requires locking the component set for write access
    pub fn require_components_and_add<T: Component>(&mut self, component: T) {
        unsafe { ComponentSetWriteGuard::lock_from_world(self.world).add(self.id, component) };
    }

    /// Attempts to remove a component, and returns the component if it
    /// was removed this way
    ///
    /// Requires locking the component set for write access
    pub fn require_components_and_pop<T: Component>(&mut self) -> Option<T> {
        ComponentSetWriteGuard::lock_from_world(self.world).soft_pop(self.id)
    }

    /// Builder variant of `lock_components_and_add`
    pub fn require_components_and_with<T: Component>(mut self, component: T) -> Self {
        self.require_components_and_add(component);
        self
    }

    /// Builder variant of `lock_components_and_pop`
    pub fn require_components_and_without<T: Component>(mut self) -> Self {
        self.require_components_and_pop::<T>();
        self
    }

    /// Destroys this entity and all components associated with it
    ///
    /// Does so by locking every component set, removing the component, then moving onto the next one.
    /// Thus if any component sets are currently locked and this is called on the same thread there will be a deadlock.
    pub fn require_all_components_and_destroy(self) {
        // First, clear out components. If we fail to use an old entity id boo hoo. If we reallocate an entity id
        // with already existing components we are in trouble
        {
            // Grab all locks and collect first, that way guards have something to reference
            // (if we were less lazy we could bundle with OwningRef)
            let locks = self.world.components.read().values().cloned().collect::<Vec<_>>();

            // Ensure we lock every single component set before moving to actually delete the components, that way this operation is kept
            // atomic
            let component_sets = locks.iter().map(|lock| lock.write()).collect::<Vec<_>>();
            component_sets
                .into_iter()
                .for_each(|mut component_set| component_set.remove(self.id.index));
        }

        self.world.entities.write().free_id(self.id);
    }
}

/// Entity handle scoped to a particular locked view.
///
/// Access is restricted to the component and singleton sets specified by the
/// view. Additional components must be accessed via broader locks or deferred
/// commands.
pub struct LockedViewEntity<'a, LockedViewRef>
where
    LockedViewRef: private::LockedViewRef<'a>,
{
    id: EntityId,
    locked_view: LockedViewRef,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, LockedViewRef> LockedViewEntity<'a, LockedViewRef>
where
    LockedViewRef: private::LockedViewRef<'a>,
{
    pub(crate) fn new(id: EntityId, locked_view: LockedViewRef) -> Self {
        Self {
            id,
            locked_view,
            _phantom: PhantomData::<&'a ()>,
        }
    }

    /// Gets the id of this entity
    pub fn id(&self) -> EntityId { self.id }

    /// Adds a component to this entity defered
    ///
    /// By defered meaning the operation will be placed on a command queue that then needs
    /// to be manually executed.
    ///
    /// Note if anything happens to the entity such as it being removed, this will do nothing
    fn add_defered<T: Component>(&self, component: T) {
        self.locked_view.as_ref().defered_updates.push(
            |(id, component), world| {
                if let Some(mut entity) = world.get_entity(id) {
                    entity.require_components_and_add(component);
                }
            },
            (self.id, component),
        );
    }

    /// Builder variant of `add_defered`
    pub fn with_defered<T: Component>(self, component: T) -> Self {
        self.add_defered(component);
        self
    }

    /// Removes a component from this entity defered
    ///
    /// By defered meaning the operation will be placed on a command queue that then needs
    /// to be manually executed.
    ///
    /// Note if anything happens to the entity such as it being removed, this will do nothing
    pub fn remove_defered<T: Component>(&self) {
        self.locked_view.as_ref().defered_updates.push(
            |id, world| {
                if let Some(mut entity) = world.get_entity(id) {
                    entity.require_components_and_pop::<T>();
                }
            },
            self.id,
        );
    }

    // Destroys this entity defered
    ///
    /// By defered meaning the operation will be placed on a command queue that then needs
    /// to be manually executed.
    ///
    /// Not this only takes a reference, so you can still do stuff for the entity, it just typically
    /// would not be long for this world if the defered update queue is being consumed
    pub fn destroy_defered(&self) {
        self.locked_view.as_ref().defered_updates.push(
            |id, world| {
                if let Some(entity) = world.get_entity(id) {
                    entity.require_all_components_and_destroy();
                }
            },
            self.id,
        );
    }
}

/// Provides read-only component access for entities inside a locked view.
pub trait LockedViewEntityComponentExt<C, S, Idx, QueryIdx>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    /// Accesses a component on this entity immutably - if it exists
    fn component<T: Component>(&self) -> Option<impl Deref<Target = T>>
    where
        LockedView<C, S>: HasComponents<T, C, Idx, QueryIdx>;

    /// Accesses a component on this entity mutably - if it exists
    fn component_mut<T: Component>(&self) -> Option<impl DerefMut<Target = T>>
    where
        LockedView<C, S>: HasComponentsMut<T, C, Idx>;
}

/// Provides mutable component access and mutation helpers within a locked view.
pub trait LockedViewEntityComponentMutExt<C, S, Idx>
where
    Self: Sized,
    C: LockedViewElements,
    S: LockedViewElements,
{
    /// Adds a component to this entity
    fn add<T: Component>(&mut self, component: T) -> impl DerefMut<Target = T>
    where
        LockedView<C, S>: HasComponentsMut<T, C, Idx>;

    /// Builder variant of `add_component`
    fn with<T: Component>(mut self, component: T) -> Self
    where
        Self: Sized,
        LockedView<C, S>: HasComponentsMut<T, C, Idx>,
    {
        self.add(component);
        self
    }

    /// Attempts to remove a component from this entity, then returns
    /// if the component was removed
    fn pop<T: Component>(&mut self) -> Option<T>
    where
        LockedView<C, S>: HasComponentsMut<T, C, Idx>;

    /// Builder version of pop, but doesn't return the component
    fn without<T: Component>(mut self) -> Self
    where
        LockedView<C, S>: HasComponentsMut<T, C, Idx>,
    {
        self.pop();
        self
    }
}

mod private {
    use super::*;
    use crate::traits::component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor};

    pub trait LockedViewRef<'a> {
        type ComponentElements: LockedViewElements;
        type SingletonElements: LockedViewElements;

        fn as_ref(&self) -> &LockedView<Self::ComponentElements, Self::SingletonElements>;
    }

    pub trait LockedViewMut<'a>: LockedViewRef<'a> {
        fn as_mut(&mut self) -> &mut LockedView<Self::ComponentElements, Self::SingletonElements>;
    }

    impl<'a, C, S, T> LockedViewRef<'a> for T
    where
        C: LockedViewElements,
        S: LockedViewElements,
        T: Deref<Target = LockedView<C, S>>,
    {
        type ComponentElements = C;
        type SingletonElements = S;

        fn as_ref(&self) -> &LockedView<C, S> { self }
    }

    impl<'a, C, S, T> LockedViewMut<'a> for T
    where
        C: LockedViewElements,
        S: LockedViewElements,
        T: DerefMut<Target = LockedView<C, S>>,
    {
        fn as_mut(&mut self) -> &mut LockedView<C, S> { &mut *self }
    }

    impl EntityId {
        /// Creates a new entity id
        pub(crate) fn new(index: usize, generation: usize) -> Self { Self { index, generation } }
    }

    impl<'a, LockedViewRef, Idx, QueryIdx>
        LockedViewEntityComponentExt<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements, Idx, QueryIdx>
        for LockedViewEntity<'a, LockedViewRef>
    where
        LockedViewRef: private::LockedViewRef<'a>,
        Idx: 'static,
        QueryIdx: 'static,
    {
        fn component<T: Component>(&self) -> Option<impl Deref<Target = T>>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponents<T, LockedViewRef::ComponentElements, Idx, QueryIdx>,
        {
            // SAFETY: The accessor originates from the locked view and enforces
            // read-only borrowing of the component set.
            unsafe { self.locked_view.as_ref().get_accessor().get(self.id) }
        }

        fn component_mut<T: Component>(&self) -> Option<impl DerefMut<Target = T>>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponentsMut<T, LockedViewRef::ComponentElements, Idx>,
        {
            // SAFETY: The accessor enforces aliasing rules for the component set
            // locked mutably by this view.
            unsafe { self.locked_view.as_ref().get_accessor().get_mut(self.id) }
        }
    }

    impl<'a, LockedViewRef, Idx>
        LockedViewEntityComponentMutExt<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements, Idx>
        for LockedViewEntity<'a, LockedViewRef>
    where
        LockedViewRef: private::LockedViewMut<'a>,
        Idx: 'static,
    {
        fn add<T: Component>(&mut self, component: T) -> impl DerefMut<Target = T>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponentsMut<T, LockedViewRef::ComponentElements, Idx>,
        {
            // SAFETY: The mutable accessor owns the component set lock, making
            // addition safe for this entity.
            unsafe { self.locked_view.as_mut().get_mut_accessor().add(self.id, component) }
        }

        fn pop<T: Component>(&mut self) -> Option<T>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponentsMut<T, LockedViewRef::ComponentElements, Idx>,
        {
            // SAFETY: Accessor enforces exclusive access, so removing the
            // component maintains invariants.
            self.locked_view.as_mut().get_mut_accessor().soft_pop(self.id)
        }
    }
}
