use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}

/// Trait for an index
pub trait Index: private::Sealed {}

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the first element.
pub struct Here;

impl private::Sealed for Here {}
impl Index for Here {}

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the latter element.
pub struct There<T: Index>(PhantomData<T>);

impl<T: Index> private::Sealed for There<T> {}
impl<T: Index> Index for There<T> {}
