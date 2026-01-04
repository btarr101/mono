use std::ops::{Deref, DerefMut};

use crate::{
    locked_view::{
        LockedView,
        has_singleton::{HasSingleton, HasSingletonMut},
        locked_view_elements::LockedViewElements,
    },
    traits::{
        singleton::Singleton,
        singleton_container_accessor::{
            MutSingletonContainerMutAccessor, SingletonContainerAccessor, SingletonContainerMutAccessor,
        },
    },
};

mod private {
    pub trait Sealed {}
}

// Extension trait go gain access to a singleton from this view
pub trait LockedViewGetSingletonExt<S: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Gets a singleton from this view
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<C, S, Idx, QueryIdx> LockedViewGetSingletonExt<S, Idx, QueryIdx> for LockedView<C, S>
where
    S: LockedViewElements,
    C: LockedViewElements,
    Idx: 'static,
    QueryIdx: 'static,
{
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>,
    {
        unsafe { self.get_accessor().get() }
    }
}

/// Entry to a singleton in a locked view
pub struct LockedViewSingletonEntry<'a, T: Singleton>(&'a mut Option<T>);

impl<'a, T: Singleton> LockedViewSingletonEntry<'a, T> {
    /// Inserts a singleton, then returns a reference to it
    pub fn insert(self, singleton: T) -> impl DerefMut<Target = T> + 'a { self.0.insert(singleton) }

    /// Inserts a singleton if there currently isn't one, then returns a reference
    /// to the newly inserted singleton or the current
    pub fn or_insert(self, default: T) -> impl DerefMut<Target = T> + 'a { self.0.get_or_insert(default) }

    /// `or_insert` but with a callback for lazy construction
    pub fn or_insert_with<F>(self, default: F) -> impl DerefMut<Target = T> + 'a
    where
        F: FnOnce() -> T,
    {
        self.0.get_or_insert_with(default)
    }

    /// Gets the singleton or inserts it with the default value
    pub fn or_default(self) -> impl DerefMut<Target = T> + 'a
    where
        T: Default,
    {
        self.0.get_or_insert_default()
    }
}

pub trait LockedViewGetSingletonMutExt<S: LockedViewElements, Idx>: private::Sealed {
    /// Gets a singleton mutably from this view
    fn get_singleton_mut<T: Singleton>(&self) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Inserts a singleton and returns an immediate reference to it
    fn insert_singleton<T: Singleton>(&mut self, singleton: T) -> impl DerefMut<Target = T>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Attempts to remove a singleton, and if a singleton is removed returns it
    fn pop_singleton<T: Singleton>(&mut self) -> Option<T>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Gets a singleton entry from this locked view
    fn singleton_entry<T: Singleton>(&mut self) -> LockedViewSingletonEntry<'_, T>
    where
        Self: HasSingletonMut<T, S, Idx>;
}

impl<C, S, Idx> LockedViewGetSingletonMutExt<S, Idx> for LockedView<C, S>
where
    S: LockedViewElements,
    C: LockedViewElements,
    Idx: 'static,
{
    fn get_singleton_mut<T: Singleton>(&self) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        unsafe { self.get_accessor().get_mut() }
    }

    fn insert_singleton<T: Singleton>(&mut self, singleton: T) -> impl DerefMut<Target = T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        unsafe { self.get_mut_accessor().insert(singleton) }
    }

    fn pop_singleton<T: Singleton>(&mut self) -> Option<T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        unsafe { self.get_mut_accessor().pop() }
    }

    fn singleton_entry<T: Singleton>(&mut self) -> LockedViewSingletonEntry<'_, T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        LockedViewSingletonEntry(unsafe { self.get_mut_accessor().get_entry() })
    }
}
