//! Type-level indices for tuple navigation.

/// Marker trait for type-level tuple indices.
pub trait Index {}

/// Index representing the first element.
pub struct Here;

/// Index representing a subsequent element.
pub struct There<T: Index>(std::marker::PhantomData<T>);

impl Index for Here {}
impl<T: Index> Index for There<T> {}
