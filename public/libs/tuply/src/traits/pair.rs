mod private {
    pub trait Sealed {}
}

/// Accesses the two elements of a 2-tuple with explicit roles.
///
/// # Invariants
/// - `First` corresponds to element `0` of the tuple.
/// - `Second` corresponds to element `1` of the tuple.
pub trait Pair: private::Sealed {
    /// First element type.
    type First;
    /// Second element type.
    type Second;

    /// Returns a shared reference to the first element.
    fn first(&self) -> &Self::First;
    /// Returns a mutable reference to the first element.
    fn first_mut(&mut self) -> &mut Self::First;
    /// Consumes and returns the first element.
    fn into_first(self) -> Self::First;

    /// Returns a shared reference to the second element.
    fn second(&self) -> &Self::Second;
    /// Returns a mutable reference to the second element.
    fn second_mut(&mut self) -> &mut Self::Second;
    /// Consumes and returns the second element.
    fn into_second(self) -> Self::Second;
}

impl<A, B> private::Sealed for (A, B) {}
impl<A, B> Pair for (A, B) {
    type First = A;
    type Second = B;

    fn first(&self) -> &Self::First { &self.0 }
    fn first_mut(&mut self) -> &mut Self::First { &mut self.0 }
    fn into_first(self) -> Self::First { self.0 }

    fn second(&self) -> &Self::Second { &self.1 }
    fn second_mut(&mut self) -> &mut Self::Second { &mut self.1 }
    fn into_second(self) -> Self::Second { self.1 }
}
