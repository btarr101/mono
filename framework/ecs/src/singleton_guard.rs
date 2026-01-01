use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use owning_ref::OwningHandle;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::traits::singleton::Singleton;

pub struct SingletonContainerReadGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<Option<T>>>, RwLockReadGuard<'static, Option<T>>>,
);

pub struct SingletonContainerWriteGuard<T: Singleton>(
    pub(crate) OwningHandle<Arc<RwLock<Option<T>>>, RwLockWriteGuard<'static, Option<T>>>,
);

impl<T: Singleton> Deref for SingletonContainerReadGuard<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Singleton> Deref for SingletonContainerWriteGuard<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Singleton> DerefMut for SingletonContainerWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
