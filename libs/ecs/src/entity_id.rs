//! Entities and entity-scoped access APIs.
//!
//! This module defines entity identifiers, entity handles, and extensions
//! for accessing components through locked views.

/// Stable identifier for an entity and its components within a `World`.
#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

impl EntityId {
    /// Creates a new entity id
    pub(crate) fn new(index: usize, generation: usize) -> Self { Self { index, generation } }
}
