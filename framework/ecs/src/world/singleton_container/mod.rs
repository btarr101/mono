use std::cell::{Ref, RefMut};

use crate::{traits::singleton::Singleton, util::danger_cell::DangerCell};

pub mod singleton_guards;

/// Internal structure for storring singletons
pub struct SingletonContainer<T: Singleton>(DangerCell<Option<T>>);

impl<T: Singleton> SingletonContainer<T> {
    /// Creates a new singleton container
    pub fn new() -> Self { Self(DangerCell::new(None)) }

    /// Gets the optional singleton
    ///
    /// # Safety
    /// Only call if no thread thinks it has exclusive access
    pub unsafe fn get_shared(&self) -> &Option<T> { unsafe { self.0.get_shared() } }

    /// Gets a singleton
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_exclusive(&self) -> Option<Ref<'_, T>> {
        Ref::filter_map(unsafe { self.0.get() }?, |borrow| borrow.as_ref()).ok()
    }

    /// Gets a singleton, but mutably
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_mut_exclusive(&self) -> Option<RefMut<'_, T>> {
        RefMut::filter_map(unsafe { self.0.get_mut() }?, |borrow| borrow.as_mut()).ok()
    }

    /// Gets a singleton, but mutably, and skips ref count checking
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn mut_get_mut_exclusive(&mut self) -> &mut Option<T> { unsafe { self.0.get_mut_exclusive() } }

    /// Inserts a singleton and immediately returns it
    ///
    /// # Safety
    /// Only call if this thread has exlcusive access
    pub unsafe fn insert(&mut self, singleton: T) -> &mut T { unsafe { self.0.get_mut_exclusive() }.insert(singleton) }

    /// Attempts to remove the singleton, and if a singleton is removed this
    /// way returns it
    pub unsafe fn pop(&mut self) -> Option<T> { unsafe { self.0.get_mut_exclusive() }.take() }
}
