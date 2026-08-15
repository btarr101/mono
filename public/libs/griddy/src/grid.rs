use std::{collections::VecDeque, iter::repeat_n};

use crate::grid_bounds::{GridBoundExt, GridBounds};

pub trait GridCell: Default + Clone {}
impl<T: Default + Clone> GridCell for T {}

/// A grid of cells
pub struct Grid<C: GridCell> {
    width: usize,
    cells: VecDeque<C>,
    top_left_offset: glam::ISizeVec2,
    default_cell: C,
}

impl<C: GridCell> Default for Grid<C> {
    fn default() -> Self { Self::new() }
}

impl<C: GridCell> Grid<C> {
    /// Creates a new empty grid
    pub fn new() -> Self {
        Self {
            top_left_offset: glam::ISizeVec2::default(),
            width: 0,
            cells: VecDeque::new(),
            default_cell: C::default(),
        }
    }

    /// Gets the width of the grid
    pub fn width(&self) -> usize { self.width }

    /// Gets the height of the grid
    pub fn height(&self) -> usize { if self.width > 0 { self.cells.len() / self.width } else { 0 } }

    /// Gets the size of the grid
    pub fn size(&self) -> (usize, usize) { (self.width(), self.height()) }

    /// Gets the minimum position of the grid
    pub fn min(&self) -> (isize, isize) { (-self.top_left_offset).into() }

    /// Gets the maximum position of the grid
    pub fn max(&self) -> (isize, isize) {
        (-self.top_left_offset + glam::isizevec2(self.width() as isize, self.height() as isize)).into()
    }

    /// Gets the bounds of the grid
    pub fn bounds(&self) -> GridBounds {
        let min = self.min();
        let max = self.max();

        (min.0..max.0, min.1..max.1)
    }

    /// Gets the cell at the given position, if it exists
    pub fn get(&self, position: impl Into<(isize, isize)>) -> &C {
        let position = position.into();

        self.bounds()
            .contains(position)
            .then(|| self.position_to_index(position).and_then(|index| self.cells.get(index)))
            .flatten()
            .unwrap_or(&self.default_cell)
    }

    /// Gets the cell at the given position mutably, if it exists
    pub fn get_mut(&mut self, position: impl Into<(isize, isize)>) -> &mut C {
        let position = position.into();

        // We must expand here because we need to provide a mutable reference to a cell,
        // not a default dummy
        self.expand_to_contain(&(position.0..position.0 + 1, position.1..position.1 + 1));
        let index = self.position_to_index(position).expect("an index after expanding");

        &mut self.cells[index]
    }

    /// Upserts a cell at the given position, returning the old cell if it was in bounds
    pub fn upsert(&mut self, position: impl Into<(isize, isize)>, cell: C) -> Option<C> {
        let position = position.into();

        let expanded = self.expand_to_contain(&(position.0..position.0 + 1, position.1..position.1 + 1));
        let index = self.position_to_index(position).expect("an index after expanding");

        let replacement = std::mem::replace(&mut self.cells[index], cell);
        (!expanded).then_some(replacement)
    }

    /// Converts a canonical grid position to an index into the grid
    fn position_to_index(&self, position: impl Into<glam::ISizeVec2>) -> Option<usize> {
        self.local_position_to_index(self.position_to_local(position.into())?)
    }

    /// Converts a canonical grid position to a local indexible position
    fn position_to_local(&self, position: glam::ISizeVec2) -> Option<glam::USizeVec2> {
        glam::USizeVec2::try_from(self.top_left_offset + position).ok()
    }

    /// Converts a local position to an index into the grid
    fn local_position_to_index(&self, position: glam::USizeVec2) -> Option<usize> {
        (position.x < self.width() && position.y < self.height()).then(|| (position.y * self.width() + position.x) as usize)
    }

    /// Expand this grid internally to contain the given positions
    ///
    /// Returns if this was expanded
    pub fn expand_to_contain(&mut self, bounds: &GridBounds) -> bool {
        let min = glam::ISizeVec2::from(self.min());
        let max = glam::ISizeVec2::from(self.max());
        let new_min = min.min((bounds.0.start, bounds.1.start).into());
        let new_max = max.max((bounds.0.end, bounds.1.end).into());

        let columns_before = usize::try_from(min.x - new_min.x).unwrap_or_default();
        let columns_after = usize::try_from(new_max.x - max.x).unwrap_or_default();
        let rows_before = usize::try_from(min.y - new_min.y).unwrap_or_default();
        let rows_after = usize::try_from(new_max.y - max.y).unwrap_or_default();

        // Bootstrap empty grid
        if self.width == 0 {
            let width = usize::try_from(new_max.x - new_min.x).unwrap_or_default();
            let height = usize::try_from(new_max.y - new_min.y).unwrap_or_default();

            if width > 0 && height > 0 {
                self.width = width;
                self.cells.extend(repeat_n(C::default(), width * height));
                self.top_left_offset = -new_min;

                return true;
            }
        }

        // Prepend rows
        self.cells.extend_front(repeat_n(C::default(), rows_before * self.width()));

        // Append rows
        self.cells.extend(repeat_n(C::default(), rows_after * self.width()));

        let rows = self.height(); // We get the height after we have added the rows

        // Prepend columns
        for row in (0..rows).rev() {
            let index = row * self.width();
            self.cells.splice(index..index, repeat_n(C::default(), columns_before));
        }
        self.width += columns_before;

        // Append columns
        for row in (0..rows).rev() {
            let index = (row + 1) * self.width();
            self.cells.splice(index..index, repeat_n(C::default(), columns_after));
        }
        self.width += columns_after;

        self.top_left_offset = -new_min;

        columns_before != 0 || columns_after != 0 || rows_before != 0 || rows_after != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_default() {
        let grid = Grid::<usize>::default();

        assert_eq!(grid.get((0, 0)), &0);
    }

    #[test]
    fn test_get_mut() {
        let mut grid = Grid::<usize>::default();
        let cell = grid.get_mut((56, -32));
        assert_eq!(cell, &mut 0);

        *cell = 42;
        assert_eq!(grid.get((56, -32)), &42);
    }

    #[test]
    fn test_upsert() {
        let mut grid = Grid::<usize>::default();

        let previous = grid.upsert((56, -32), 42);

        assert_eq!(previous, None);
        assert_eq!(grid.get((56, -32)), &42);
    }
}
