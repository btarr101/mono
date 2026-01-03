use std::ops::{Deref, DerefMut};

use tuples::{index::Here, traits::has::ConsHas};

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView, has_components::HasComponents, locked_view_elements::LockedViewElements, types::ConsComponentSetGuards,
    },
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor},
        component_tuple_element::ComponentTupleElement,
    },
    world::component_set::component_set_guards::ComponentSetWriteGuard,
};

/// An element used in a query tuple for a locked view query
pub trait LockedViewQueryElement<'a, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx>: ComponentTupleElement {
    /// The accessors that can be used to iterate over components for this C
    type Accessors;

    /// The type of the borrow for the component (which depends on what accessor was used)
    type BorrowedComponent;

    /// Gets the correct accessor for a component set from this locked view and iterates across it
    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a;
}

impl<'a, C, S, Idx, QueryIdx, T: Component> LockedViewQueryElement<'a, C, S, Idx, QueryIdx> for &'a T
where
    C: LockedViewElements + 'a,
    S: LockedViewElements + 'a,
    Idx: 'static,
    QueryIdx: 'static,
    LockedView<C, S>: HasComponents<Self::Component, C, Idx, QueryIdx>,
{
    type Accessors = ConsComponentSetGuards<T>;
    type BorrowedComponent = impl Deref<Target = T>;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        unsafe { view.get_accessor().iter() }
    }
}

impl<'a, C, S, Idx, T: Component> LockedViewQueryElement<'a, C, S, Idx, Here> for &mut T
where
    C: LockedViewElements + 'a,
    C::ComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
    S: LockedViewElements + 'a,
    Idx: 'static,
{
    type Accessors = ConsComponentSetGuards<T>;
    type BorrowedComponent = impl DerefMut<Target = T>;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        unsafe { view.components.cons_get_ref().iter_mut() }
    }
}
