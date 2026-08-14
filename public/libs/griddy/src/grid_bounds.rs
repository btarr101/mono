use std::ops::Range;

use crate::sealed::Sealed;

/// Represents the bounds of a grid, as a pair of `Range`s for the x and y axes
pub type GridBounds = (Range<isize>, Range<isize>);

pub trait GridBoundExt: Sealed {
    /// Returns if the point is within the bounds
    fn contains(&self, position: impl Into<(isize, isize)>) -> bool;
}

impl Sealed for GridBounds {}
impl GridBoundExt for GridBounds {
    fn contains(&self, position: impl Into<(isize, isize)>) -> bool {
        let (x, y) = position.into();
        self.0.contains(&x) && self.1.contains(&y)
    }
}
