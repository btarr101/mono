use std::marker::PhantomData;

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the first element.
pub struct Here;

/// Never intended to be constructed. Type for an index into
/// a cons like tuple representing the latter element.
pub struct There<T: Index>(PhantomData<T>);

pub trait Index: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for Here {}
impl Index for Here {}

impl<T: Index> sealed::Sealed for There<T> {}
impl<T: Index> Index for There<T> {}
