use std::ops::{Deref, DerefMut};

use crate::{
    component_set_guards::{ComponentSetGuard, ComponentSetWriteGuard},
    locked_view::{
        LockedView,
        private::{HasComponents, HasComponentsMut, LockedViewElements},
    },
    traits::{component::Component, component_set_accessor::MutComponentSetMutAccessor},
    world::World,
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
        ComponentSetWriteGuard::lock_from_world(self.world).add(self.id, component);
    }

    /// Attempts to remove a component, and returns the component if it
    /// was removed this way
    ///
    /// Requires locking the component set for write access
    pub fn lock_components_and_pop<T: Component>(&mut self) -> Option<T> {
        ComponentSetWriteGuard::lock_from_world(self.world).soft_pop(self.id)
    }

    /// Builder variant of `lock_components_and_add`
    pub fn lock_components_and_with<T: Component>(mut self, component: T) -> Self {
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
    pub fn lock_all_components_and_destroy(self) {
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
pub struct LockedViewEntity<'a, Elements: LockedViewElements> {
    id: EntityId,
    locked_view: &'a mut LockedView<Elements>,
}

impl<'a, Elements: LockedViewElements> LockedViewEntity<'a, Elements> {
    pub(crate) fn new(id: EntityId, locked_view: &'a mut LockedView<Elements>) -> Self { Self { id, locked_view } }

    /// Gets the id of this entity
    pub fn id(&self) -> EntityId { self.id }
}

/// Extension trait for a locked view entity to access a component immutably
pub trait LockedViewEntityComponentExt<Elements: LockedViewElements, Idx, QueryIdx> {
    /// Accesses a component on this entity immutably - if it exists
    fn component<T: Component>(&self) -> Option<impl Deref<Target = T>>
    where
        LockedView<Elements>: HasComponents<T, Elements, Idx, QueryIdx>;
}

/// Extension trait for a locked view entity to access a component mutably,
/// or to add or remove a component
pub trait LockedViewEntityComponentMutExt<Elements: LockedViewElements, Idx>
where
    Self: Sized,
{
    /// Accesses a component on this entity mutably - if it exists
    fn component_mut<T: Component>(&self) -> Option<impl DerefMut<Target = T>>
    where
        LockedView<Elements>: HasComponentsMut<T, Elements, Idx>;

    /// Adds a component to this entity
    fn add<T: Component>(&mut self, component: T) -> impl DerefMut<Target = T>
    where
        LockedView<Elements>: HasComponentsMut<T, Elements, Idx>;

    /// Builder variant of `add_component`
    fn with<T: Component>(mut self, component: T) -> Self
    where
        Self: Sized,
        LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
    {
        self.add(component);
        self
    }

    /// Attempts to remove a component from this entity, then returns
    /// if the component was removed
    fn pop<T: Component>(&mut self) -> Option<T>
    where
        LockedView<Elements>: HasComponentsMut<T, Elements, Idx>;
}

mod private {
    use super::*;
    use crate::traits::component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor};

    impl EntityId {
        /// Creates a new entity id
        pub(crate) fn new(index: usize, generation: usize) -> Self { Self { index, generation } }
    }

    impl<'a, Elements, Idx, QueryIdx> LockedViewEntityComponentExt<Elements, Idx, QueryIdx> for LockedViewEntity<'a, Elements>
    where
        Elements: LockedViewElements,
        Idx: 'static,
        QueryIdx: 'static,
    {
        fn component<T: Component>(&self) -> Option<impl Deref<Target = T>>
        where
            LockedView<Elements>: HasComponents<T, Elements, Idx, QueryIdx>,
        {
            self.locked_view.get_accessor().get(self.id)
        }
    }

    impl<'a, Elements, Idx> LockedViewEntityComponentMutExt<Elements, Idx> for LockedViewEntity<'a, Elements>
    where
        Elements: LockedViewElements,
        Idx: 'static,
    {
        fn component_mut<T: Component>(&self) -> Option<impl DerefMut<Target = T>>
        where
            LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
        {
            self.locked_view.get_accessor().get_mut(self.id)
        }

        fn add<T: Component>(&mut self, component: T) -> impl DerefMut<Target = T>
        where
            LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
        {
            self.locked_view.get_mut_accessor().add(self.id, component)
        }

        fn pop<T: Component>(&mut self) -> Option<T>
        where
            LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
        {
            self.locked_view.get_mut_accessor().soft_pop(self.id)
        }
    }
}
