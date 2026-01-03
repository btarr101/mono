use std::ops::{Deref, DerefMut};

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView,
        has_components::{HasComponents, HasComponentsMut},
        locked_view_elements::LockedViewElements,
    },
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
    },
};

mod private {
    pub trait Sealed {}
}

/// Extension trait go gain access to a component from this view
pub trait LockedViewGetComponentExt<C: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Gets a component associated with an entity from this view
    fn get_component<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: HasComponents<T, C, Idx, QueryIdx>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<C, S, Idx, QueryIdx> LockedViewGetComponentExt<C, Idx, QueryIdx> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idx: 'static,
    QueryIdx: 'static,
{
    fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: HasComponents<T, C, Idx, QueryIdx>,
    {
        unsafe { self.get_accessor().get(entity_id) }
    }
}

/// Extension trait go gain access to a component mutably from this view
pub trait LockedViewGetComponentMutExt<C: LockedViewElements, Idx>: private::Sealed {
    /// Gets a component associated with an entity mutably from this view
    fn get_component_mut<T: Component>(&self, id: EntityId) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>;

    /// Attempts to add a component to an entity in this view
    ///
    /// Marked as must use, as checking the operation was successful is as simple an ensuring the option is some
    #[must_use]
    fn add_component<T: Component>(&mut self, id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>;

    /// Attempts to a remove component from an entity,
    /// if a component is removed this way returns it
    fn pop_component<T: Component>(&mut self, id: EntityId) -> Option<T>
    where
        Self: HasComponentsMut<T, C, Idx>;
}

impl<C, S, Idx> LockedViewGetComponentMutExt<C, Idx> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idx: 'static,
{
    fn get_component_mut<T: Component>(&self, entity_id: EntityId) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        unsafe { self.get_accessor().get_mut(entity_id) }
    }

    fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        self.get_mut_accessor().try_add(entity_id, component)
    }

    fn pop_component<T: Component>(&mut self, entity_id: EntityId) -> Option<T>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        self.get_mut_accessor().soft_pop(entity_id)
    }
}
