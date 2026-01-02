use crate::{
    singleton_guards::{OptionalSingletonContainerReadGuard, OptionalSingletonContainerWriteGuard, SingletonContainerGuard},
    traits::singleton::Singleton,
};

mod private {
    pub trait Sealed {}
}

/// An element used as a specifier in a locked view
pub trait SingletonTupleElement: private::Sealed {
    type Singleton: Singleton;
    type Guard: SingletonContainerGuard<Singleton = Self::Singleton>;
}
impl<T: Singleton> private::Sealed for &T {}
impl<T: Singleton> SingletonTupleElement for &T {
    type Singleton = T;
    type Guard = OptionalSingletonContainerReadGuard<T>;
}
impl<T: Singleton> private::Sealed for &mut T {}
impl<T: Singleton> SingletonTupleElement for &mut T {
    type Singleton = T;
    type Guard = OptionalSingletonContainerWriteGuard<T>;
}
