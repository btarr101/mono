use crate::{
    sparse_set_entry::{SparseSetEntry, SparseSetOccupiedEntry, SparseSetVacantEntry},
    sparse_set_index::SparseSetIndex,
};

/// A sparse set, which is a data structure that stores key value pairs where
/// the key is an index and the value is anything.
///
/// Useful because it assumes the size of the key is much smaller than the size of the value
/// so storring values where indices may be sparse or spread apart should have minimal consequence.
pub struct SparseSet<T, I: SparseSetIndex = usize> {
    /// Mapping from sparse index to dense index
    pub(crate) sparse: Vec<Option<usize>>,
    /// Contains the raw data as well as the full sparse set index
    pub(crate) dense: Vec<(I, T)>,
}

impl<T, I: SparseSetIndex> SparseSet<T, I> {
    /// Creates a new sparse set
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    /// Inserts a new element, returning the index of the previous element
    /// and the element if present
    pub fn insert(&mut self, index: I, value: T) -> Option<(I, T)> {
        let raw_index = index.index();
        if raw_index >= self.sparse.len() {
            self.sparse.resize(raw_index + 1, None);
        }

        if let Some(dense_index) = self.sparse[raw_index] {
            let old_entry = std::mem::replace(&mut self.dense[dense_index], (index, value));
            return Some(old_entry);
        }

        let dense_index = self.dense.len();
        self.dense.push((index, value));
        self.sparse[raw_index] = Some(dense_index);

        None
    }

    /// Gets an entry for in-place manipulation.
    pub fn entry(&mut self, index: I) -> SparseSetEntry<'_, T, I> {
        if let Some(dense_index) = self.get_dense_index(&index)
            && self
                .dense
                .get(dense_index)
                .is_some_and(|(current_index, _)| *current_index == index)
        {
            SparseSetEntry::Occupied(SparseSetOccupiedEntry { set: self, dense_index })
        } else {
            SparseSetEntry::Vacant(SparseSetVacantEntry { set: self, index })
        }
    }

    /// Gets an element given an index
    pub fn get(&self, index: I) -> Option<&T> {
        let dense_index = self.get_dense_index(&index)?;
        self.dense
            .get(dense_index)
            .and_then(|(current_index, value)| (*current_index == index).then_some(value))
    }

    /// Returns if this sparse set contains an index
    pub fn contains(&self, index: I) -> bool { self.get(index).is_some() }

    /// Returns the number of elements in the sparse set
    pub fn len(&self) -> usize { self.dense.len() }

    /// Returns if this sparse set is empty
    pub fn is_empty(&self) -> bool { self.dense.is_empty() }

    /// Iterates over all entries in the sparse set, in order of index
    pub fn iter(&self) -> impl Iterator<Item = (&I, &T)> + '_ {
        self.sparse.iter().filter_map(|dense_index| {
            dense_index.map(|dense_index| {
                let entry = &self.dense[dense_index];
                (&entry.0, &entry.1)
            })
        })
    }

    /// Iterates over all entries in the sparse set mutably, in order of index
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&I, &mut T)> + '_ {
        let dense_ptr = self.dense.as_mut_ptr();

        self.sparse
            .iter()
            .filter_map(|dense_index| *dense_index)
            .map(move |dense_index| {
                // SAFETY:
                // - `dense_index` comes from `self.sparse`, which only stores valid
                //   indices into `self.dense`.
                // - Each dense index occurs at most once in a valid sparse set, so
                //   the iterator never yields overlapping mutable references.
                let entry = unsafe { &mut *dense_ptr.add(dense_index) };

                let (index, value) = entry;
                (&*index, value)
            })
    }

    /// Gets an element mutably given an index
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        let dense_index = self.get_dense_index(&index)?;
        self.dense
            .get_mut(dense_index)
            .and_then(|(current_index, value)| (*current_index == index).then_some(value))
    }

    /// Removes an element given it's index (if it is present)
    pub fn pop(&mut self, index: I) -> Option<T> {
        let raw_index = index.index();
        let dense_index = self.sparse.get_mut(raw_index).and_then(Option::take)?;

        let (_, removed_value) = self.dense.swap_remove(dense_index);

        // If swap_remove moved another element into `dense_index`, update its sparse pointer.
        if let Some((moved_sparse_index, _)) = self.dense.get(dense_index) {
            self.sparse[moved_sparse_index.index()] = Some(dense_index);
        }

        Some(removed_value)
    }

    /// Clears the sparse set
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
    }

    fn get_dense_index(&self, index: &I) -> Option<usize> { self.sparse.get(index.index()).and_then(|entry| *entry) }
}

impl<T, I: SparseSetIndex> Default for SparseSet<T, I> {
    fn default() -> Self { Self::new() }
}

impl<T, I: SparseSetIndex> FromIterator<(I, T)> for SparseSet<T, I> {
    fn from_iter<Iter: IntoIterator<Item = (I, T)>>(iter: Iter) -> Self {
        let mut set = Self::new();

        for (index, value) in iter {
            set.insert(index, value);
        }

        set
    }
}

impl<T, I: SparseSetIndex, const N: usize> From<[(I, T); N]> for SparseSet<T, I> {
    fn from(entries: [(I, T); N]) -> Self { entries.into_iter().collect() }
}

impl<T, I: SparseSetIndex> IntoIterator for SparseSet<T, I> {
    type Item = (I, T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut dense = self.dense.into_iter().map(Some).collect::<Vec<_>>();
        self.sparse
            .into_iter()
            .filter_map(move |dense_index| dense_index.and_then(|dense_index| dense[dense_index].take()))
    }
}
