//! Accessor traits for component set guards.

use std::ops::{Deref, DerefMut};

use crate::{
    entity::EntityId,
    traits::component::Component,
    world::component_set::component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
};

mod private {
    pub trait Sealed<T> {}
}

/// Trait used to access a component set immutably
pub trait ComponentSetAccessor<T: Component>: private::Sealed<T> {
    type BorrowedComponent<'a>: Deref<Target = T> + 'a
    where
        Self: 'a;

    /// Gets a component from this component set given an entity id
    unsafe fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>>;

    /// Iterates through this component set returning pairs of entity id and component.
    unsafe fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)>;
}

impl<T: Component> private::Sealed<T> for ComponentSetReadGuard<T> {}
impl<T: Component> ComponentSetAccessor<T> for ComponentSetReadGuard<T> {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    unsafe fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> {
        // SAFETY: Read guard ensures only shared access to components.
        unsafe { self.0.get_shared(id) }
    }
    unsafe fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> {
        // SAFETY: Shared iteration is valid while the read guard is held.
        unsafe { self.0.iter_shared() }
    }
}

impl<T: Component> private::Sealed<T> for ComponentSetWriteGuard<T> {}
impl<T: Component> ComponentSetAccessor<T> for ComponentSetWriteGuard<T> {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    unsafe fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> {
        // SAFETY: Write guard holds exclusive access to the component set.
        unsafe { self.0.get_exclusive(id) }
    }
    unsafe fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> {
        // SAFETY: Exclusive access allows iterating with unique borrows.
        unsafe { self.0.iter_exclusive() }
    }
}

impl<T: Component, A: ComponentSetAccessor<T>> private::Sealed<T> for &A {}
impl<T: Component, A: ComponentSetAccessor<T>> ComponentSetAccessor<T> for &A {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    unsafe fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).get(id) }
    }
    unsafe fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).iter() }
    }
}

impl<T: Component, A: ComponentSetAccessor<T>> private::Sealed<T> for &mut A {}
impl<T: Component, A: ComponentSetAccessor<T>> ComponentSetAccessor<T> for &mut A {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    unsafe fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).get(id) }
    }
    unsafe fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).iter() }
    }
}

/// Trait used to access the components in a component set mutably
pub trait ComponentSetMutAccessor<T: Component>: ComponentSetAccessor<T> {
    /// Gets a mutable component from this component set given an entity id
    unsafe fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>>;

    /// Iterates through this component set returning pairs of entity id and mutable component.
    unsafe fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)>;
}

impl<T: Component> ComponentSetMutAccessor<T> for ComponentSetWriteGuard<T> {
    unsafe fn get_mut(&self, id: EntityId) -> Option<impl std::ops::DerefMut<Target = T>> {
        // SAFETY: Write guard provides exclusive access for mutable borrows.
        unsafe { self.0.get_mut_exclusive(id) }
    }
    unsafe fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl std::ops::DerefMut<Target = T>)> {
        // SAFETY: Exclusive access permits mutable iteration over all components.
        unsafe { self.0.iter_mut_exclusive() }
    }
}

impl<T: Component, A: ComponentSetMutAccessor<T>> ComponentSetMutAccessor<T> for &A {
    unsafe fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).get_mut(id) }
    }
    unsafe fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).iter_mut() }
    }
}

impl<T: Component, A: ComponentSetMutAccessor<T>> ComponentSetMutAccessor<T> for &mut A {
    unsafe fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).get_mut(id) }
    }
    unsafe fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).iter_mut() }
    }
}

/// Trait used to access a full component set mutably
pub trait MutComponentSetMutAccessor<T: Component>: ComponentSetMutAccessor<T> {
    /// Adds a component to this component set given an entity id, then returns an
    /// immediate reference to it
    unsafe fn add(&mut self, id: EntityId, component: T) -> &mut T;

    /// Attempts to add a component given an entity id, but if the generation doesn't
    /// match up does not.
    ///
    /// If added, returns an immediate reference
    unsafe fn try_add(&mut self, id: EntityId, component: T) -> Option<&mut T>;

    /// Attempts to remove a component from this component set, and ignores
    /// generation
    fn pop(&mut self, id: EntityId) -> Option<T>;

    /// Attempts to remove a component from this component set given an entity id,
    /// then returns if a component was removed this way
    fn soft_pop(&mut self, id: EntityId) -> Option<T>;
}

impl<T: Component> MutComponentSetMutAccessor<T> for ComponentSetWriteGuard<T> {
    unsafe fn add(&mut self, id: EntityId, component: T) -> &mut T {
        // SAFETY: Write guard owns exclusive access, matching `ComponentSet::add` requirements.
        unsafe { self.0.add(id, component) }
    }
    unsafe fn try_add(&mut self, id: EntityId, component: T) -> Option<&mut T> {
        // SAFETY: Write guard owns exclusive access, matching `ComponentSet::try_add` requirements.
        unsafe { self.0.try_add(id, component) }
    }
    fn pop(&mut self, id: EntityId) -> Option<T> {
        self.0.pop(id.index).and_then(|(_, component)| component)
    }
    fn soft_pop(&mut self, id: EntityId) -> Option<T> {
        self.0.soft_pop(id)
    }
}

impl<T: Component, A: MutComponentSetMutAccessor<T>> MutComponentSetMutAccessor<T> for &mut A {
    unsafe fn add(&mut self, id: EntityId, component: T) -> &mut T {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).add(id, component) }
    }
    unsafe fn try_add(&mut self, id: EntityId, component: T) -> Option<&mut T> {
        // SAFETY: Delegates to the underlying accessor, preserving its guarantees.
        unsafe { (**self).try_add(id, component) }
    }
    fn pop(&mut self, id: EntityId) -> Option<T> {
        (**self).pop(id)
    }
    fn soft_pop(&mut self, id: EntityId) -> Option<T> {
        (**self).soft_pop(id)
    }
}
