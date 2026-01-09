use std::ops::Deref;

use derived_deref::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockReadGuard};

pub trait MaybeLocked<T> {
    fn maybe_read<'a>(&'a self) -> Read<'a, T>;
}

#[derive(Debug, Deref, DerefMut)]
pub struct Unlocked<T>(T);
impl<T> Unlocked<T> {
    pub fn new(data: T) -> Self { Self(data) }
}

impl<T> MaybeLocked<T> for Unlocked<T> {
    fn maybe_read<'a>(&'a self) -> Read<'a, T> { Read::Plain(self) }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Locked<T>(RwLock<T>);
impl<T> Locked<T> {
    pub fn new(data: T) -> Self { Self(data.into()) }
}

impl<T> MaybeLocked<T> for Locked<T> {
    fn maybe_read<'a>(&'a self) -> Read<'a, T> { Read::Locked(self.read(), |t| t) }
}

pub enum Read<'a, T, U = T, P: Fn(&U) -> &T = fn(&U) -> &T> {
    Plain(&'a T),
    Locked(RwLockReadGuard<'a, U>, P),
}

impl<'a, T, U, P: Fn(&U) -> &T> Deref for Read<'a, T, U, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Read::Plain(data) => data,
            Read::Locked(guard, projector) => projector(guard),
        }
    }
}

impl<'a, T: 'static, U, P: Fn(&U) -> &T> Read<'a, T, U, P> {
    pub fn map<V, VP: Fn(&T) -> &V>(self, mapper: VP) -> Read<'a, V, U, impl Fn(&U) -> &V> {
        match self {
            Read::Plain(data) => Read::Plain(mapper(data)),
            Read::Locked(guard, projector) => Read::Locked(guard, move |u| mapper(projector(u))),
        }
    }
}
