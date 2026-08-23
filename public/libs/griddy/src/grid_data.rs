/// Raw grid data that can be loaded into a grid
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridData<C> {
    /// Width of the grid
    pub width: usize,
    /// Cells in the grid
    pub cells: Vec<C>,
    /// World position of the top-left cell
    #[cfg_attr(feature = "serde", serde(default))]
    pub top_left_offset: (isize, isize),
}

impl<C> Default for GridData<C> {
    fn default() -> Self {
        Self {
            width: 0,
            cells: Vec::new(),
            top_left_offset: (0, 0),
        }
    }
}
