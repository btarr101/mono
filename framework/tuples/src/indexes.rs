use std::marker::PhantomData;

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the first element.
pub struct Here;

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the latter element.
pub struct There<T: Index>(PhantomData<T>);

pub trait Index: private::Sealed {}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for Here {}
impl Index for Here {}

impl<T: Index> private::Sealed for There<T> {}
impl<T: Index> Index for There<T> {}
