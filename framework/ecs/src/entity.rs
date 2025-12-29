#[derive(Clone, Copy, Debug)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

impl EntityId {
    /// Creates a new entity id
    pub fn new(index: usize, generation: usize) -> Self { Self { index, generation } }
}
