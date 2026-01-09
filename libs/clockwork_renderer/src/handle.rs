use std::{ops::Deref, sync::Arc};

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

use crate::maybe_locked::{Locked, MaybeLocked, Read, Unlocked};

pub struct Handle<'a, T>(Arc<dyn MaybeLocked<T> + 'a>);

impl<'a, T> Clone for Handle<'a, T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<'a, T> Handle<'a, T> {
    pub fn maybe_read(&'a self) -> Read<'a, T> { self.0.maybe_read() }
}

impl<'a, T: 'a> From<UnlockedHandle<T>> for Handle<'a, T> {
    fn from(value: UnlockedHandle<T>) -> Self { Self(value.0) }
}

impl<'a, T: 'a> From<LockedHandle<T>> for Handle<'a, T> {
    fn from(value: LockedHandle<T>) -> Self { Self(value.0) }
}

pub struct UnlockedHandle<T>(Arc<Unlocked<T>>);

impl<T> Clone for UnlockedHandle<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T> Deref for UnlockedHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> UnlockedHandle<T> {
    pub fn new(data: T) -> Self { Self(Arc::new(Unlocked::new(data))) }
}

pub struct LockedHandle<T>(Arc<Locked<T>>);

impl<T> Clone for LockedHandle<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T> LockedHandle<T> {
    pub fn new(data: T) -> Self { Self(Arc::new(Locked::new(data))) }
    pub fn read(&self) -> RwLockReadGuard<'_, T> { self.0.read() }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> { self.0.write() }
}
