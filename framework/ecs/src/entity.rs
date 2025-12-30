use std::ops::{Deref, DerefMut};

use crate::{
    locked_view::{
        LockedView,
        private::{HasComponents, HasComponentsMut, LockedViewElements},
    },
    traits::component::Component,
};

/// An identifier for an entity (and each of its components)
#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
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
    fn add<T: Component>(&mut self, component: T)
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
    use crate::{
        locked_view::{LockedViewComponentsExt, LockedViewComponentsMutExt},
        traits::component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
    };

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
            self.locked_view.components().get(self.id)
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
            self.locked_view.components_mut().get_mut(self.id)
        }

        fn add<T: Component>(&mut self, component: T)
        where
            LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
        {
            self.locked_view.mut_components_mut().add(self.id, component);
        }

        fn pop<T: Component>(&mut self) -> Option<T>
        where
            LockedView<Elements>: HasComponentsMut<T, Elements, Idx>,
        {
            self.locked_view.mut_components_mut().pop(self.id)
        }
    }
}
