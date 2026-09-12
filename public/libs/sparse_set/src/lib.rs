#![feature(impl_trait_in_assoc_type)]

pub mod sparse_set;
pub mod sparse_set_entry;
pub mod sparse_set_index;

pub mod prelude {
    pub use super::{
        sparse_set::SparseSet,
        sparse_set_entry::{SparseSetEntry, SparseSetOccupiedEntry, SparseSetVacantEntry},
        sparse_set_index::SparseSetIndex,
    };
}
