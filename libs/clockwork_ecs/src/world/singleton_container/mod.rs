//! Storage for singleton values within a world.

use std::cell::{Ref, RefMut};

use crate::{traits::singleton::Singleton, util::danger_cell::DangerCell};

pub mod singleton_guards;

/// DangerCell-backed storage for a single optional value.
pub struct SingletonContainer<T: Singleton>(DangerCell<Option<T>>);

impl<T: Singleton> SingletonContainer<T> {
    /// Creates a new singleton container
    pub fn new() -> Self { Self(DangerCell::new(None)) }

    /// Gets the optional singleton
    ///
    /// # Safety
    /// Only call if no thread thinks it has exclusive access
    pub unsafe fn get_shared(&self) -> &Option<T> {
        // SAFETY: Caller guarantees no exclusive access while observing the option.
        unsafe { self.0.get_shared() }
    }

    /// Gets a singleton
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_exclusive(&self) -> Option<Ref<'_, T>> {
        Ref::filter_map(
            {
                // SAFETY: Caller guarantees exclusive access to the DangerCell borrow.
                unsafe { self.0.get() }?
            },
            |borrow| borrow.as_ref(),
        )
        .ok()
    }

    /// Gets a singleton, but mutably
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_mut_exclusive(&self) -> Option<RefMut<'_, T>> {
        RefMut::filter_map(
            {
                // SAFETY: Exclusive access allows mutable borrows from the DangerCell.
                unsafe { self.0.get_mut() }?
            },
            |borrow| borrow.as_mut(),
        )
        .ok()
    }

    /// Gets a singleton, but mutably, and skips ref count checking
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn mut_get_mut_exclusive(&mut self) -> &mut Option<T> {
        // SAFETY: The caller holds &mut self, guaranteeing exclusive access.
        unsafe { self.0.get_mut_exclusive() }
    }

    /// Inserts a singleton and immediately returns it
    ///
    /// # Safety
    /// Only call if this thread has exlcusive access
    pub unsafe fn insert(&mut self, singleton: T) -> &mut T {
        // SAFETY: Exclusive access ensures the inner option can be mutated safely.
        unsafe { self.0.get_mut_exclusive() }.insert(singleton)
    }

    /// Attempts to remove the singleton, and if a singleton is removed this
    /// way returns it
    pub unsafe fn pop(&mut self) -> Option<T> {
        // SAFETY: Exclusive access allows taking ownership of the stored singleton.
        unsafe { self.0.get_mut_exclusive() }.take()
    }
}
