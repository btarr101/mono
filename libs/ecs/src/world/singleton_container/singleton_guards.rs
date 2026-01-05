//! Guard types for accessing singleton containers.

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use owning_ref::OwningHandle;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{traits::singleton::Singleton, util::wrap::Wrap, world::singleton_container::SingletonContainer};

/// Read-only guard providing access to a singleton value.
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
            RwLockReadGuard::try_map(
                {
                    // SAFETY: `OwningHandle` guarantees `lock` remains valid for the guard lifetime.
                    unsafe { &*lock }.read()
                },
                |container| {
                    // SAFETY: Shared guard ensures only immutable access to the singleton value.
                    unsafe { container.get_shared().as_ref() }
                },
            )
        })
        .ok()
        .map(Self)
    }
}

/// Read-only guard over the container storing an optional singleton.
pub struct OptionalSingletonContainerReadGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, RwLockReadGuard<'static, SingletonContainer<T>>>,
);

impl<T: Singleton> OptionalSingletonContainerReadGuard<T> {
    /// Creates this read guard from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| {
            // SAFETY: `OwningHandle` upholds the lifetime of the raw pointer.
            unsafe { &*lock }.read()
        }))
    }
}

/// Exclusive guard providing mutable access to a singleton value.
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
            RwLockWriteGuard::try_map(
                {
                    // SAFETY: `OwningHandle` keeps the pointer alive for the guard duration.
                    unsafe { &*lock }.write()
                },
                |container| {
                    // SAFETY: Exclusive guard allows mutable access to the singleton slot.
                    unsafe { container.mut_get_mut_exclusive().as_mut() }
                },
            )
        })
        .ok()
        .map(Self)
    }
}

/// Exclusive guard over the container storing an optional singleton.
pub struct OptionalSingletonContainerWriteGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<SingletonContainer<T>>>, RwLockWriteGuard<'static, SingletonContainer<T>>>,
);

impl<T: Singleton> OptionalSingletonContainerWriteGuard<T> {
    /// Creates this read guard from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| {
            // SAFETY: Ownership of the raw pointer is tied to the OwningHandle instance.
            unsafe { &*lock }.write()
        }))
    }
}

/// Lazy-initialization entry for a singleton slot within the world.
pub struct SingletonContainerEntry<T: Singleton>(
    #[expect(clippy::type_complexity)]
    pub(crate)  OwningHandle<Arc<RwLock<SingletonContainer<T>>>, Wrap<Option<RwLockWriteGuard<'static, SingletonContainer<T>>>>>,
);

impl<T: Singleton> SingletonContainerEntry<T> {
    /// Creates this singleton container entry from the world
    pub(crate) fn from_lock(lock: Arc<RwLock<SingletonContainer<T>>>) -> Self {
        Self(OwningHandle::new_with_fn(lock, |lock| {
            // SAFETY: The raw pointer remains valid for the lifetime of the owning handle.
            Wrap(Some(unsafe { &*lock }.write()))
        }))
    }

    /// Inserts a new singleton into the entry, then returns an occupied entry
    pub fn insert(mut self, singleton: T) -> OccupiedSingletonContainerEntry<T> {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        // SAFETY: The guard holds exclusive access to the singleton slot.
        unsafe { guard.insert(singleton) };
        std::mem::forget(guard);

        // SAFETY: We intentionally leaked the previous guard to keep the lock alive for the new handle.
        unsafe { self.into_occupied_entry() }
    }

    /// Inserts a new singleton into the entry if there isn't one, then returns what is in
    /// the entry
    pub fn or_insert(mut self, default: T) -> OccupiedSingletonContainerEntry<T> {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        // SAFETY: The guard holds exclusive access to the singleton slot while inserting.
        unsafe { guard.mut_get_mut_exclusive().get_or_insert(default) };
        std::mem::forget(guard);

        // SAFETY: The leaked guard preserves the logical lock for the new handle.
        unsafe { self.into_occupied_entry() }
    }

    /// `or_insert` but with a callback
    pub fn or_insert_with<F>(mut self, default: F) -> OccupiedSingletonContainerEntry<T>
    where
        F: FnOnce() -> T,
    {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        // SAFETY: Exclusive access allows initializing the singleton lazily.
        unsafe { guard.mut_get_mut_exclusive().get_or_insert_with(default) };
        std::mem::forget(guard);

        // SAFETY: The leaked guard preserves the logical lock for the new handle.
        unsafe { self.into_occupied_entry() }
    }

    /// If there isn't a singleton, inserts the default, then returns an occupied entry
    pub fn or_default(mut self) -> OccupiedSingletonContainerEntry<T>
    where
        T: Default,
    {
        // Here we insert the value into the guard, then we forget it so the data remains locked while we build our new lock guard.
        let mut guard = self.0.take().expect("some");
        // SAFETY: Exclusive access allows default-initializing the singleton slot.
        unsafe { guard.mut_get_mut_exclusive().get_or_insert_default() };
        std::mem::forget(guard);

        // SAFETY: The leaked guard preserves the logical lock for the new handle.
        unsafe { self.into_occupied_entry() }
    }

    unsafe fn into_occupied_entry(self) -> OccupiedSingletonContainerEntry<T> {
        OccupiedSingletonContainerEntry(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can call `make_write_guard_unchecked` because we ensure that we still logically hold a write lock by
            // making the previous write guard forget to clean up
            Wrap(Some(RwLockWriteGuard::map(
                {
                    // SAFETY: The forgotten guard ensures the lock remains held, permitting unchecked creation.
                    unsafe { (&*lock).make_write_guard_unchecked() }
                },
                |guard| {
                    // SAFETY: We still hold exclusive access when projecting into the singleton slot.
                    unsafe { guard.mut_get_mut_exclusive() }.as_mut().expect("some")
                },
            )))
        }))
    }
}

/// Entry that already owns an initialized singleton.
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
        let guard = {
            // SAFETY: The previous write guard was forgotten, so we still logically hold the lock.
            unsafe { self.0.as_owner().make_write_guard_unchecked() }
        };
        std::mem::forget(RwLockWriteGuard::downgrade(guard));

        SingletonContainerReadGuard(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can make an unchecked read guard here, because above we forgot a downgraded write guard
            RwLockReadGuard::map(
                {
                    // SAFETY: The lock remains held, permitting unchecked guard creation.
                    unsafe { (&*lock).make_read_guard_unchecked() }
                },
                |container| {
                    // SAFETY: Shared guard exposes only immutable access to the singleton.
                    unsafe { container.get_shared() }.as_ref().expect("some")
                },
            )
        }))
    }

    /// Keeps exlcusive access to this singleton and creates a write guard
    pub fn write(mut self) -> SingletonContainerWriteGuard<T> {
        // Forget the current mapped rwlock write guard
        std::mem::forget(self.0.take().expect("some"));

        SingletonContainerWriteGuard(OwningHandle::new_with_fn(self.0.into_owner(), |lock| {
            // We can make an unchecked read guard here, because above we forgot a downgraded write guard
            RwLockWriteGuard::map(
                {
                    // SAFETY: The lock remains held, permitting unchecked guard creation.
                    unsafe { (&*lock).make_write_guard_unchecked() }
                },
                |container| {
                    // SAFETY: Exclusive guard allows mutable access to the singleton slot.
                    unsafe { container.mut_get_mut_exclusive() }.as_mut().expect("some")
                },
            )
        }))
    }
}
