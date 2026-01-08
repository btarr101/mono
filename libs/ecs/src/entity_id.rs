//! Entities and entity-scoped access APIs.
//!
//! This module defines entity identifiers, entity handles, and extensions
//! for accessing components through locked views.

/// Stable identifier for an entity and its components within a [`World`](crate::world::World).
///
/// # Examples
/// ```
/// use ecs::world::World;
///
/// let world = World::new();
/// let entity = world.create_entity();
///
/// assert!(world.entity_exists(entity));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

impl EntityId {
    /// Creates a new entity id
    pub(crate) fn new(index: usize, generation: usize) -> Self { Self { index, generation } }
}
