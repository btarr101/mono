use std::fmt::Debug;

/// Collection that stores sparsely populated indices densely in memory.
pub struct SparseSet<T> {
    pub sparse_to_dense: Vec<Option<usize>>,
    dense: Vec<(usize, T)>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse_to_dense: Vec::new(),
            dense: Vec::new(),
        }
    }
}

impl<T: Debug> Debug for SparseSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the set as a struct with the number of dense entries and
        // a list of (index, &T) pairs in sparse order for readability.
        let entries: Vec<(usize, &T)> = self.iter().collect();
        f.debug_struct("SparseSet")
            .field("len", &self.dense.len())
            .field("entries", &entries)
            .finish()
    }
}

#[allow(clippy::new_without_default)]
impl<T> SparseSet<T> {
    /// Creates a new empty sparse set
    pub fn new() -> Self {
        Self {
            sparse_to_dense: Vec::new(),
            dense: Vec::new(),
        }
    }

    /// Adds an element to this sparse set, then returns a reference
    /// to it
    pub fn add(&mut self, index: usize, element: T) -> &mut T {
        let dense_index = self.dense.len();
        self.dense.push((index, element));
        if index >= self.sparse_to_dense.len() {
            self.sparse_to_dense.resize(index + 1, None);
        }
        self.sparse_to_dense[index] = Some(dense_index);
        &mut self.dense[dense_index].1
    }

    /// Gets a reference to an element from this sparse set
    pub fn get(&self, index: usize) -> Option<&T> {
        let dense_index = self.sparse_to_dense.get(index).cloned().flatten()?;
        Some(&self.dense[dense_index].1)
    }

    /// Gets mutable reference to an element from this sparse set
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let dense_index = self.sparse_to_dense.get(index).cloned().flatten()?;
        Some(&mut self.dense[dense_index].1)
    }

    /// Checks if this sparse set has an element
    pub fn has(&self, index: usize) -> bool { self.sparse_to_dense.get(index).cloned().flatten().is_some() }

    /// Attempts to removes an element from this sparse set,
    /// then returns if an element was removed this way
    pub fn pop(&mut self, index: usize) -> Option<T> {
        let dense_index = self.sparse_to_dense.get(index).cloned().flatten()?;

        self.sparse_to_dense[index] = None;

        let last_dense = self.dense.pop().expect("last element");

        if dense_index < self.dense.len() {
            self.sparse_to_dense[last_dense.0] = Some(dense_index);
            std::mem::replace(&mut self.dense[dense_index], last_dense).1.into()
        } else {
            last_dense.1.into()
        }
    }

    /// Iterates through this sparse set
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        // TODO: This is gross, we iterate through sparse entries to get dense entries just
        // to maintain sorted iteration (since dense entries can become unsorted with the current
        // swap + delete implementation).
        self.sparse_to_dense.iter().filter_map(|sparse| {
            sparse
                .map(|dense| self.dense.get(dense).map(|(index, data)| (*index, data)))
                .flatten()
        })
    }

    /// Iterates through this sparse set mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.dense.iter_mut().map(|(index, element)| (*index, element))
    }

    /// Clears this spsarse set
    pub fn clear(&mut self) {
        self.sparse_to_dense.clear();
        self.dense.clear();
    }

    /// Checks if this sparse set is empty
    pub fn is_empty(&self) -> bool { self.dense.is_empty() }

    /// Gets an element or the default
    pub fn get_or_default(&mut self, index: usize) -> &T
    where
        T: Default,
    {
        if let Some(dense_index) = self.sparse_to_dense.get(index).cloned().flatten() {
            &self.dense[dense_index].1
        } else {
            self.add(index, T::default());
            self.get(index).expect("just inserted")
        }
    }

    /// Gets an element or the default mutably
    pub fn get_or_default_mut(&mut self, index: usize) -> &mut T
    where
        T: Default,
    {
        if let Some(dense_index) = self.sparse_to_dense.get(index).cloned().flatten() {
            &mut self.dense[dense_index].1
        } else {
            self.add(index, T::default());
            self.get_mut(index).expect("just inserted")
        }
    }
}
