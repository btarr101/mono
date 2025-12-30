use std::ops::{Deref, DerefMut};

use crate::{
    component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    entity::EntityId,
    traits::component::Component,
};

mod sealed {
    pub trait Sealed<T> {}
}

/// Trait used to access a component set immutably
pub trait ComponentSetAccessor<T: Component>: sealed::Sealed<T> {
    type BorrowedComponent<'a>: Deref<Target = T> + 'a
    where
        Self: 'a;

    /// Gets a component from this component set given an entity id
    fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>>;

    /// Iterates through this component set returning pairs of entity id and component.
    fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)>;
}

impl<T: Component> sealed::Sealed<T> for ComponentSetReadGuard<T> {}
impl<T: Component> ComponentSetAccessor<T> for ComponentSetReadGuard<T> {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> { unsafe { self.0.get_shared(id) } }
    fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> { unsafe { self.0.iter_shared() } }
}

impl<T: Component> sealed::Sealed<T> for ComponentSetWriteGuard<T> {}
impl<T: Component> ComponentSetAccessor<T> for ComponentSetWriteGuard<T> {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> { unsafe { self.0.get_exclusive(id) } }
    fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> { unsafe { self.0.iter_exclusive() } }
}

impl<T: Component, A: ComponentSetAccessor<T>> sealed::Sealed<T> for &A {}
impl<T: Component, A: ComponentSetAccessor<T>> ComponentSetAccessor<T> for &A {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> { (**self).get(id) }
    fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> { (**self).iter() }
}

impl<T: Component, A: ComponentSetAccessor<T>> sealed::Sealed<T> for &mut A {}
impl<T: Component, A: ComponentSetAccessor<T>> ComponentSetAccessor<T> for &mut A {
    type BorrowedComponent<'a>
        = impl Deref<Target = T> + 'a
    where
        Self: 'a;

    fn get(&self, id: EntityId) -> Option<Self::BorrowedComponent<'_>> { (**self).get(id) }
    fn iter(&self) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent<'_>)> { (**self).iter() }
}

/// Trait used to access the components in a component set mutably
pub trait ComponentSetMutAccessor<T: Component>: ComponentSetAccessor<T> {
    /// Gets a mutable component from this component set given an entity id
    fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>>;

    /// Iterates through this component set returning pairs of entity id and mutable component.
    fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)>;
}

impl<T: Component> ComponentSetMutAccessor<T> for ComponentSetWriteGuard<T> {
    fn get_mut(&self, id: EntityId) -> Option<impl std::ops::DerefMut<Target = T>> { unsafe { self.0.get_mut_exclusive(id) } }
    fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl std::ops::DerefMut<Target = T>)> {
        unsafe { self.0.iter_mut_exclusive() }
    }
}

impl<T: Component, A: ComponentSetMutAccessor<T>> ComponentSetMutAccessor<T> for &A {
    fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>> { (**self).get_mut(id) }
    fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)> { (**self).iter_mut() }
}

impl<T: Component, A: ComponentSetMutAccessor<T>> ComponentSetMutAccessor<T> for &mut A {
    fn get_mut(&self, id: EntityId) -> Option<impl DerefMut<Target = T>> { (**self).get_mut(id) }
    fn iter_mut(&self) -> impl Iterator<Item = (EntityId, impl DerefMut<Target = T>)> { (**self).iter_mut() }
}

/// Trait used to access a full component set mutably
pub trait MutComponentSetMutAccessor<T: Component>: ComponentSetMutAccessor<T> {
    /// Adds a comonent to this component set given an entity id
    fn add(&mut self, id: EntityId, component: T);

    /// Attempts to remove a component from this component set given an entity id,
    /// then returns if a component was removed this way
    fn pop(&mut self, id: EntityId) -> Option<T>;
}

impl<T: Component> MutComponentSetMutAccessor<T> for ComponentSetWriteGuard<T> {
    fn add(&mut self, id: EntityId, component: T) { self.0.add(id, component); }
    fn pop(&mut self, id: EntityId) -> Option<T> { self.0.pop(id) }
}

impl<T: Component, A: MutComponentSetMutAccessor<T>> MutComponentSetMutAccessor<T> for &mut A {
    fn add(&mut self, id: EntityId, component: T) { (**self).add(id, component); }
    fn pop(&mut self, id: EntityId) -> Option<T> { (**self).pop(id) }
}
