use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use owning_ref::OwningHandle;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{traits::singleton::Singleton, util::wrap::Wrap, world::singleton_container::SingletonContainer};

/// Guard that sits in front of a singleton (gives read only access)
pub struct SingletonContainerReadGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, MappedRwLockReadGuard<'static, T>>,
);

impl<T: Singleton> Deref for SingletonContainerReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Singleton> SingletonContainerReadGuard<T> {
    /// Creates this read guard from the world
    pub(crate) fn try_from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Option<Self> {
        OwningHandle::try_new(lock, |lock| {
            RwLockReadGuard::try_map(unsafe { &*lock }.read(), |container| unsafe {
                container.get_shared().as_ref()
            })
        })
        .ok()
        .map(Self)
    }
}

/// Guard that sits in front of a singleton and the option it is enclosed in
pub struct OptionalSingletonContainerReadGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, RwLockReadGuard<'static, SingletonContainer<T>>>,
);

impl<T: Singleton> OptionalSingletonContainerReadGuard<T> {
    /// Creates this read guard from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.read()))
    }
}

/// Guard that sits in front of a singleton (gives write)
pub struct SingletonContainerWriteGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, MappedRwLockWriteGuard<'static, T>>,
);

impl<T: Singleton> Deref for SingletonContainerWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Singleton> DerefMut for SingletonContainerWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: Singleton> SingletonContainerWriteGuard<T> {
    /// Creates this write guard from the world
    pub(crate) fn try_from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Option<Self> {
        OwningHandle::try_new(lock, |lock| {
            RwLockWriteGuard::try_map(unsafe { &*lock }.write(), |container| unsafe {
                container.mut_get_mut_exclusive().as_mut()
            })
        })
        .ok()
        .map(Self)
    }
}

/// Guard that sits in front of a singleton and the option it is enclosed in
pub struct OptionalSingletonContainerWriteGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, RwLockWriteGuard<'static, SingletonContainer<T>>>,
);

impl<T: Singleton> OptionalSingletonContainerWriteGuard<T> {
    /// Creates this read guard from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| unsafe { &*lock }.write()))
    }
}

/// Gets an entry to a singleton in the world
pub struct SingletonContainerEntry<T: Singleton>(
    #[expect(clippy::type_complexity)]
    pub(crate)  OwningHandle<Arc<RwLock<SingletonContainer<T>>>, Wrap<Option<RwLockWriteGuard<'static, SingletonContainer<T>>>>>,
);

impl<T: Singleton> SingletonContainerEntry<T> {
    /// Creates this singleton container entry from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| Wrap(Some(unsafe { &*lock }.write()))))
    }

    /// Inserts a new singleton into the entry, then returns an occupied entry
    pub fn insert(mut self, singleton: T) -> OccupiedSingletonContainerEntry<T> {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        unsafe { guard.insert(singleton) };
        std::mem::forget(guard);

        unsafe { self.into_occupied_entry() }
    }

    /// Inserts a new singleton into the entry if there isn't one, then returns what is in
    /// the entry
    pub fn or_insert(mut self, default: T) -> OccupiedSingletonContainerEntry<T> {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        unsafe { guard.mut_get_mut_exclusive().get_or_insert(default) };
        std::mem::forget(guard);

        unsafe { self.into_occupied_entry() }
    }

    /// `or_insert` but with a callback
    pub fn or_insert_with<F>(mut self, default: F) -> OccupiedSingletonContainerEntry<T>
    where
        F: FnOnce() -> T,
    {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        unsafe { guard.mut_get_mut_exclusive().get_or_insert_with(default) };
        std::mem::forget(guard);

        unsafe { self.into_occupied_entry() }
    }

    /// If there isn't a singleton, inserts the default, then returns an occupied entry
    pub fn or_default(mut self) -> OccupiedSingletonContainerEntry<T>
    where
        T: Default,
    {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        unsafe { guard.mut_get_mut_exclusive().get_or_insert_default() };
        std::mem::forget(guard);

        unsafe { self.into_occupied_entry() }
    }

    unsafe fn into_occupied_entry(self) -> OccupiedSingletonContainerEntry<T> {
        OccupiedSingletonContainerEntry(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can call `make_write_guard_unchecked` because we ensure that we still logically hold a write lock by
            // making the previous write guard forget to clean up
            Wrap(Some(RwLockWriteGuard::map(
                unsafe { (&*lock).make_write_guard_unchecked() },
                |guard| unsafe { guard.mut_get_mut_exclusive() }.as_mut().expect("some"),
            )))
        }))
    }
}

/// Gets an entry to a singleton in the world
pub struct OccupiedSingletonContainerEntry<T: Singleton>(
    #[expect(clippy::type_complexity)]
    pub(crate)  OwningHandle<Arc<RwLock<SingletonContainer<T>>>, Wrap<Option<MappedRwLockWriteGuard<'static, T>>>>,
);

impl<T: Singleton> OccupiedSingletonContainerEntry<T> {
    /// Downgrades exclusive access to this singleton and creates a shared read guard
    pub fn read(mut self) -> SingletonContainerReadGuard<T> {
        // Forget the current mapped rwlock write guard
        std::mem::forget(self.0.take().expect("some"));

        // Build a new rwlockwrite guard, downgrade it, then forget it
        let guard = unsafe { self.0.as_owner().make_write_guard_unchecked() };
        std::mem::forget(RwLockWriteGuard::downgrade(guard));

        SingletonContainerReadGuard(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can make an unchecked read guard here, because above we forgot a downgraded write guard
            RwLockReadGuard::map(unsafe { (&*lock).make_read_guard_unchecked() }, |container| {
                unsafe { container.get_shared() }.as_ref().expect("some")
            })
        }))
    }

    /// Keeps exlcusive access to this singleton and creates a write guard
    pub fn write(mut self) -> SingletonContainerWriteGuard<T> {
        // Forget the current mapped rwlock write guard
        std::mem::forget(self.0.take().expect("some"));

        SingletonContainerWriteGuard(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can make an unchecked read guard here, because above we forgot a downgraded write guard
            RwLockWriteGuard::map(unsafe { (&*lock).make_write_guard_unchecked() }, |container| {
                unsafe { container.mut_get_mut_exclusive() }.as_mut().expect("some")
            })
        }))
    }
}
