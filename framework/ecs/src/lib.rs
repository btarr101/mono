#![feature(unsafe_cell_access)]
#![feature(impl_trait_in_assoc_type)]

pub(crate) mod component_set;
pub(crate) mod component_set_guards;
pub(crate) mod danger_cell;
pub mod entity;
pub(crate) mod entity_id_allocator;
pub mod locked_view;
pub(crate) mod singleton_guard;
pub(crate) mod sorted_type_arcmap;
pub(crate) mod sparse_set;
pub mod traits;
pub mod world;
