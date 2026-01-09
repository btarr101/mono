use std::{ops::Deref, sync::Arc};

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

use crate::maybe_locked::{Locked, MaybeLocked, Read, Unlocked};

#[derive(Clone)]
pub struct Handle<'a, T>(Arc<dyn MaybeLocked<T> + 'a>);

impl<'a, T> Handle<'a, T> {
    pub fn maybe_read(&'a self) -> Read<'a, T> { self.0.maybe_read() }
}

impl<'a, T: 'a> From<UnlockedHandle<T>> for Handle<'a, T> {
    fn from(value: UnlockedHandle<T>) -> Self { Self(value.0) }
}

impl<'a, T: 'a> From<LockedHandle<T>> for Handle<'a, T> {
    fn from(value: LockedHandle<T>) -> Self { Self(value.0) }
}

#[derive(Clone)]
pub struct UnlockedHandle<T>(Arc<Unlocked<T>>);

impl<T> Deref for UnlockedHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> UnlockedHandle<T> {
    pub fn new(data: T) -> Self { Self(Arc::new(Unlocked::new(data))) }
}

#[derive(Clone)]
pub struct LockedHandle<T>(Arc<Locked<T>>);

impl<T> LockedHandle<T> {
    pub fn new(data: T) -> Self { Self(Arc::new(Locked::new(data))) }
    pub fn read(&self) -> RwLockReadGuard<'_, T> { self.0.read() }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> { self.0.write() }
}
