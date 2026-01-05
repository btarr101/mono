//! Query element helpers for locked view iteration.

use std::ops::{Deref, DerefMut};

use clockwork_tuples::{index::Here, traits::has::ConsHas};

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView,
        has_components::{HasComponents, HasComponentsMut},
        has_singleton::{HasSingleton, HasSingletonMut},
        locked_view_elements::LockedViewElements,
        types::{ConsComponentSetGuards, ConsSingletonContainerGuards},
    },
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor},
        component_tuple_element::ComponentTupleElement,
        singleton::Singleton,
        singleton_container_accessor::{SingletonContainerAccessor, SingletonContainerMutAccessor},
        singleton_tuple_element::SingletonTupleElement,
    },
    world::{
        component_set::component_set_guards::ComponentSetWriteGuard,
        singleton_container::singleton_guards::OptionalSingletonContainerWriteGuard,
    },
};

/// Describes a query element that borrows components from a `LockedView`.
pub trait LockedViewComponentQueryElement<'a, T: LockedViewElements, S: LockedViewElements, Idx, QueryIdx>:
    ComponentTupleElement
{
    /// The accessors that can be used to iterate over components
    type ComponentAccessors;

    /// The type of the borrow for the component (which depends on what accessor was used)
    type BorrowedComponent;

    /// Gets the correct accessor for a component set from this locked view and iterates across it
    fn iter_locked_view(view: &'a LockedView<T, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a;
}

impl<'a, C, S, Idx, QueryIdx, T: Component> LockedViewComponentQueryElement<'a, C, S, Idx, QueryIdx> for &'a T
where
    C: LockedViewElements + 'a,
    S: LockedViewElements + 'a,
    Idx: 'static,
    QueryIdx: 'static,
    LockedView<C, S>: HasComponents<Self::Component, C, Idx, QueryIdx>,
{
    type ComponentAccessors = ConsComponentSetGuards<T>;
    type BorrowedComponent = impl Deref<Target = T>;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        // SAFETY: `HasComponents` guarantees the accessor references the locked component set.
        unsafe { view.get_accessor().iter() }
    }
}

impl<'a, C, S, Idx, T: Component> LockedViewComponentQueryElement<'a, C, S, Idx, Here> for &mut T
where
    C: LockedViewElements + 'a,
    S: LockedViewElements + 'a,
    Idx: 'static,
    C::ComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
{
    type ComponentAccessors = ConsComponentSetGuards<T>;
    type BorrowedComponent = impl DerefMut<Target = T>;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        // SAFETY: Mutable access is guarded by the component set lock in the view.
        unsafe { view.get_accessor().iter_mut() }
    }
}

/// Describes a query element that borrows singletons from a `LockedView`.
pub trait LockedViewSingletonQueryElement<'a, T: LockedViewElements, S: LockedViewElements, Idx, QueryIdx>:
    SingletonTupleElement
{
    /// The accessors that can be used to iterate over singletons
    type SingletonAccessors;

    /// The type of the borrow for the singleton (which depends on what accessor was used)
    type SingletonRowElement;

    /// Gets the singleton row element
    ///
    /// `None` indicates this singleton isn't present, and the entire query should be "cancelled"
    /// Some() indicates to no cancel the query
    fn get_singleton_row_element(view: &'a LockedView<T, S>) -> Option<Self::SingletonRowElement>;
}

impl<'a, C, S, Idx, QueryIdx, T: Singleton> LockedViewSingletonQueryElement<'a, C, S, Idx, QueryIdx> for &'a T
where
    C: LockedViewElements + 'a,
    S: LockedViewElements + 'a,
    Idx: 'static,
    QueryIdx: 'static,
    LockedView<C, S>: HasSingleton<Self::Singleton, S, Idx, QueryIdx>,
{
    type SingletonAccessors = ConsSingletonContainerGuards<T>;
    type SingletonRowElement = impl Deref<Target = T>;

    fn get_singleton_row_element(view: &'a LockedView<C, S>) -> Option<Self::SingletonRowElement> {
        // SAFETY: `HasSingleton` guarantees the accessor references the locked singleton container.
        unsafe { view.get_accessor().get() }
    }
}

impl<'a, C, S, Idx, T: Singleton> LockedViewSingletonQueryElement<'a, C, S, Idx, Here> for &'a mut T
where
    C: LockedViewElements + 'a,
    S: LockedViewElements + 'a,
    Idx: 'static,
    S::SingletonContainerGuards: ConsHas<OptionalSingletonContainerWriteGuard<T>, Idx>,
{
    type SingletonAccessors = ConsSingletonContainerGuards<T>;
    type SingletonRowElement = impl DerefMut<Target = T>;

    fn get_singleton_row_element(view: &'a LockedView<C, S>) -> Option<Self::SingletonRowElement> {
        // SAFETY: `HasSingletonMut` ensures exclusive access to the singleton container.
        unsafe { view.get_accessor().get_mut() }
    }
}
