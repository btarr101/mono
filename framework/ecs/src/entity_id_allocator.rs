use crate::entity::EntityId;

/// An allocator for entity ids (which are generation indexes)
/// ```
#[derive(Default)]
pub struct EntityIdAllocator {
    freed_indexes: Vec<usize>,
    next_index: usize,
    generation: usize,
}

impl EntityIdAllocator {
    /// Creates a new allocator
    pub fn new() -> Self { Self::default() }

    /// Allocates an entity id
    pub fn allocate_id(&mut self) -> EntityId {
        let index = self.freed_indexes.pop().unwrap_or_else(|| {
            let index = self.next_index;
            self.next_index += 1;
            index
        });

        EntityId::new(index, self.generation)
    }

    /// Frees an entity id, hinting that its index may be re-used
    /// for allocating further ids (but not its generation)
    pub fn free_id(&mut self, id: EntityId) {
        self.generation += 1;
        self.freed_indexes.push(id.index);
    }

    /// Checks if the index portion of an entity id is currently being used
    pub fn index_in_use(&self, index: usize) -> bool { !(index >= self.next_index || self.freed_indexes.contains(&index)) }
}
