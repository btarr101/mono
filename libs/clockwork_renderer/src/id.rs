use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub struct Id<T> {
    index: usize,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("Id<{}>", std::any::type_name::<T>());
        f.debug_struct(&name).field("index", &self.index).finish()
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool { self.index == other.index }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.index.hash(state) }
}
