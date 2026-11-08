use crate::{sparse_set::SparseSet, sparse_set_index::SparseSetIndex};

/// A view into a single entry in a [`SparseSet`].
pub enum SparseSetEntry<'a, T, I: SparseSetIndex = usize> {
    /// An entry whose index is present in the sparse set.
    Occupied(SparseSetOccupiedEntry<'a, T, I>),
    /// An entry whose index is not present in the sparse set.
    Vacant(SparseSetVacantEntry<'a, T, I>),
}

impl<'a, T, I: SparseSetIndex> SparseSetEntry<'a, T, I> {
    /// Returns the index associated with this entry.
    pub fn key(&self) -> &I {
        match self {
            SparseSetEntry::Occupied(entry) => entry.key(),
            SparseSetEntry::Vacant(entry) => entry.key(),
        }
    }

    /// Returns a mutable reference to the existing value, or inserts `default`.
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            SparseSetEntry::Occupied(entry) => entry.into_mut(),
            SparseSetEntry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Returns a mutable reference to the existing value, or inserts a lazily computed value.
    ///
    /// `default` is called only when the entry is vacant.
    pub fn or_insert_with(self, default: impl FnOnce() -> T) -> &'a mut T {
        match self {
            SparseSetEntry::Occupied(entry) => entry.into_mut(),
            SparseSetEntry::Vacant(entry) => entry.insert(default()),
        }
    }
}

/// A view into an occupied entry in a [`SparseSet`].
pub struct SparseSetOccupiedEntry<'a, T, I: SparseSetIndex = usize> {
    pub(crate) set: &'a mut SparseSet<T, I>,
    pub(crate) dense_index: usize,
}

impl<'a, T, I: SparseSetIndex> SparseSetOccupiedEntry<'a, T, I> {
    /// Returns the index associated with this entry.
    pub fn key(&self) -> &I { &self.set.dense[self.dense_index].0 }

    /// Returns a shared reference to the entry's value.
    pub fn get(&self) -> &T { &self.set.dense[self.dense_index].1 }

    /// Returns a mutable reference to the entry's value.
    pub fn get_mut(&mut self) -> &mut T { &mut self.set.dense[self.dense_index].1 }

    /// Converts this entry into a mutable reference with the sparse set's lifetime.
    pub fn into_mut(self) -> &'a mut T { &mut self.set.dense[self.dense_index].1 }

    /// Replaces the entry's value and returns the previous value.
    pub fn insert(&mut self, value: T) -> T { std::mem::replace(&mut self.set.dense[self.dense_index].1, value) }
}

/// A view into a vacant entry in a [`SparseSet`].
pub struct SparseSetVacantEntry<'a, T, I: SparseSetIndex = usize> {
    pub(crate) set: &'a mut SparseSet<T, I>,
    pub(crate) index: I,
}

impl<'a, T, I: SparseSetIndex> SparseSetVacantEntry<'a, T, I> {
    /// Returns the index associated with this entry.
    pub fn key(&self) -> &I { &self.index }

    /// Inserts a value and returns a mutable reference to it.
    pub fn insert(self, value: T) -> &'a mut T {
        let raw_index = self.index.index();
        self.set.insert(self.index, value);
        let dense_index = self.set.sparse[raw_index].expect("inserted entry");
        &mut self.set.dense[dense_index].1
    }
}
