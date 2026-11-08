/// Trait for a type that can be used to index
/// into a sparse set
///
/// NOTE: No two entries in a sparse set can share
/// a sparse set index that have the same `index`.
pub trait SparseSetIndex: PartialEq {
    fn index(&self) -> usize;
}

impl SparseSetIndex for usize {
    fn index(&self) -> usize { *self }
}
