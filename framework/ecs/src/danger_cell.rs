use std::cell::{Ref, RefCell, RefMut};

use static_assertions::assert_impl_all;

assert_impl_all!(DangerCell<u32>: Send, Sync);

/// A very dangerous blow your foot off cell type
///
/// # Motive
/// When a thread has read only access to the underlying data, that means
/// - No other thread should have write access
/// - Threads should feel completely free to create as many read references as they see fit
///
/// When a thread has write only access to the underlying data, that means
/// - No other thread has write access
///
/// However, a feature we want to implement is runtime borrow checking on the same thread. (ie) if
/// a user attempts to get a read reference when there already exists a write reference, or a write
/// reference if there already exists a read reference or another write reference.
///
/// Essentially, in shared mode this can just be the plain ol' data, however in exclusive mode we
/// want runtime borrow checking.
pub struct DangerCell<T>(RefCell<T>);

unsafe impl<T> Sync for DangerCell<T> {}
impl<T> DangerCell<T> {
    pub fn new(data: T) -> Self { Self(data.into()) }

    /// Consumes this cell and returns the inner value
    pub fn into_inner(self) -> T { self.0.into_inner() }

    /// # Safety
    /// Only call if no thread thinks it has exclusive access
    pub unsafe fn get_shared(&self) -> &T { unsafe { &*self.0.as_ptr() } }

    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get(&self) -> Option<Ref<'_, T>> { self.0.try_borrow().ok() }

    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_mut(&self) -> Option<RefMut<'_, T>> { self.0.try_borrow_mut().ok() }
}

impl<T> From<T> for DangerCell<T> {
    fn from(data: T) -> Self { Self::new(data) }
}
