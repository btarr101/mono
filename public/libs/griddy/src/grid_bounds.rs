use std::ops::Range;

use crate::sealed::Sealed;

/// Represents the bounds of a grid, as a pair of `Range`s for the x and y axes
pub type GridBounds = (Range<isize>, Range<isize>);

pub trait GridBoundExt: Sealed {
    /// Returns if the point is within the bounds
    fn contains(&self, position: impl Into<(isize, isize)>) -> bool;

    /// Returns the bounds shifted by the given offset
    fn shifted_by(&self, offset: (isize, isize)) -> Self;
}

impl Sealed for GridBounds {}
impl GridBoundExt for GridBounds {
    fn contains(&self, position: impl Into<(isize, isize)>) -> bool {
        let (x, y) = position.into();
        self.0.contains(&x) && self.1.contains(&y)
    }

    fn shifted_by(&self, offset: (isize, isize)) -> Self {
        (
            self.0.start + offset.0..self.0.end + offset.0,
            self.1.start + offset.1..self.1.end + offset.1,
        )
    }
}
