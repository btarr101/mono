use std::ops::{Deref, DerefMut};

use crate::{
    traits::singleton::Singleton,
    world::singleton_container::singleton_guards::{OptionalSingletonContainerReadGuard, OptionalSingletonContainerWriteGuard},
};

mod private {
    pub trait Sealed<T> {}
}

/// Trait used to access a singleton container
pub trait SingletonContainerAccessor<T: Singleton>: private::Sealed<T> {
    /// Gets a singleton from this container
    unsafe fn get(&self) -> Option<impl Deref<Target = T>>;
}

impl<T: Singleton> private::Sealed<T> for OptionalSingletonContainerReadGuard<T> {}
impl<T: Singleton> SingletonContainerAccessor<T> for OptionalSingletonContainerReadGuard<T> {
    unsafe fn get(&self) -> Option<impl Deref<Target = T>> { unsafe { self.0.get_shared() }.as_ref() }
}

impl<T: Singleton> private::Sealed<T> for OptionalSingletonContainerWriteGuard<T> {}
impl<T: Singleton> SingletonContainerAccessor<T> for OptionalSingletonContainerWriteGuard<T> {
    unsafe fn get(&self) -> Option<impl Deref<Target = T>> { unsafe { self.0.get_exclusive() } }
}

impl<T: Singleton, A: SingletonContainerAccessor<T>> private::Sealed<T> for &A {}
impl<T: Singleton, A: SingletonContainerAccessor<T>> SingletonContainerAccessor<T> for &A {
    unsafe fn get(&self) -> Option<impl Deref<Target = T>> { unsafe { (**self).get() } }
}

impl<T: Singleton, A: SingletonContainerAccessor<T>> private::Sealed<T> for &mut A {}
impl<T: Singleton, A: SingletonContainerAccessor<T>> SingletonContainerAccessor<T> for &mut A {
    unsafe fn get(&self) -> Option<impl Deref<Target = T>> { unsafe { (**self).get() } }
}

/// Trait used to access a singleton container mutably
pub trait SingletonContainerMutAccessor<T: Singleton>: SingletonContainerAccessor<T> {
    /// Gets a singleton from this container
    unsafe fn get_mut(&self) -> Option<impl DerefMut<Target = T>>;
}

impl<T: Singleton> SingletonContainerMutAccessor<T> for OptionalSingletonContainerWriteGuard<T> {
    unsafe fn get_mut(&self) -> Option<impl DerefMut<Target = T>> { unsafe { self.0.get_mut_exclusive() } }
}

impl<T: Singleton, A: SingletonContainerMutAccessor<T>> SingletonContainerMutAccessor<T> for &A {
    unsafe fn get_mut(&self) -> Option<impl DerefMut<Target = T>> { unsafe { (**self).get_mut() } }
}

impl<T: Singleton, A: SingletonContainerMutAccessor<T>> SingletonContainerMutAccessor<T> for &mut A {
    unsafe fn get_mut(&self) -> Option<impl DerefMut<Target = T>> { unsafe { (**self).get_mut() } }
}

pub trait MutSingletonContainerMutAccessor<T: Singleton>: SingletonContainerMutAccessor<T> {
    /// Inserts a singleton into this container, then returns an immediate reference to it
    unsafe fn insert(&mut self, singleton: T) -> impl DerefMut<Target = T>;

    /// Attempts to remove a singleton, if a singleton is removed this way returns it
    unsafe fn pop(&mut self) -> Option<T>;

    /// Gets the mutable entry
    unsafe fn get_entry(&mut self) -> &mut Option<T>;
}

impl<T: Singleton> MutSingletonContainerMutAccessor<T> for OptionalSingletonContainerWriteGuard<T> {
    unsafe fn insert(&mut self, singleton: T) -> impl DerefMut<Target = T> { unsafe { self.0.insert(singleton) } }
    unsafe fn pop(&mut self) -> Option<T> { unsafe { self.0.pop() } }
    unsafe fn get_entry(&mut self) -> &mut Option<T> { unsafe { self.0.mut_get_mut_exclusive() } }
}

impl<T: Singleton, A: MutSingletonContainerMutAccessor<T>> MutSingletonContainerMutAccessor<T> for &mut A {
    unsafe fn insert(&mut self, singleton: T) -> impl DerefMut<Target = T> { unsafe { (**self).insert(singleton) } }
    unsafe fn pop(&mut self) -> Option<T> { unsafe { (**self).pop() } }
    unsafe fn get_entry(&mut self) -> &mut Option<T> { unsafe { (**self).get_entry() } }
}
