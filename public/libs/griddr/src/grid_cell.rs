/// Trait for what can be a cell in a grid
pub trait GridCell: Default + Clone {}

impl<T: Default + Clone> GridCell for T {}
