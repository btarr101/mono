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

/// An identifier for an entity (and each of its components)
#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

/// An entity that references an ecs world
///
/// Note, this entity as no methods to get an immutable or mutable reference
/// type to one of its components.
pub struct Entity<'a> {
    id: EntityId,
    world: &'a World,
}

impl<'a> Entity<'a> {
    pub(crate) fn new(id: EntityId, world: &'a World) -> Self { Self { id, world } }

    /// Adds a component to this enity
    ///
    /// Requires locking the component set for write access
    pub fn lock_components_and_add<T: Component>(&mut self, component: T) {
        unsafe { ComponentSetWriteGuard::lock_from_world(self.world).add(self.id, component) };
    }

    /// Attempts to remove a component, and returns the component if it
    /// was removed this way
    ///
    /// Requires locking the component set for write access
    pub fn lock_components_and_pop<T: Component>(&mut self) -> Option<T> {
        ComponentSetWriteGuard::lock_from_world(self.world).soft_pop(self.id)
    }

    /// Builder variant of `lock_components_and_add`
    pub fn require_components_and_with<T: Component>(mut self, component: T) -> Self {
        self.lock_components_and_add(component);
        self
    }

    /// Builder variant of `lock_components_and_pop`
    pub fn lock_components_and_without<T: Component>(mut self) -> Self {
        self.lock_components_and_pop::<T>();
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

/// Access to an entity that is contained within a locked view
///
/// The entity may have components outside of the view, but it is impossible
/// to read or write to them. If you need to create entities with more components
/// then this view has, you have 2 options:
/// - Lock all the needed components, but this may results in overlocking
/// - Command buffer system, where you buffer creating entities and then create them in another locked view
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
}

/// Extension trait for a locked view entity to access a component immutably
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

/// Extension trait for a locked view entity to access a component mutably,
/// or to add or remove a component
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
            unsafe { self.locked_view.as_ref().get_accessor().get(self.id) }
        }

        fn component_mut<T: Component>(&self) -> Option<impl DerefMut<Target = T>>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponentsMut<T, LockedViewRef::ComponentElements, Idx>,
        {
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
            unsafe { self.locked_view.as_mut().get_mut_accessor().add(self.id, component) }
        }

        fn pop<T: Component>(&mut self) -> Option<T>
        where
            LockedView<LockedViewRef::ComponentElements, LockedViewRef::SingletonElements>:
                HasComponentsMut<T, LockedViewRef::ComponentElements, Idx>,
        {
            self.locked_view.as_mut().get_mut_accessor().soft_pop(self.id)
        }
    }
}
