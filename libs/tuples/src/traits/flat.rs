use crate::traits::can_prepend::CanPrepend;

/// Flattens a nested cons-style tuple into a standard tuple.
///
/// # Invariants
/// - Element order is preserved.
/// - The flattened type contains no nested tuples.
pub trait ToFlat {
    /// Resulting flat tuple type.
    type Flattened: CanPrepend;

    /// Returns the flattened tuple.
    fn flatten(self) -> Self::Flattened;
}

impl ToFlat for () {
    type Flattened = ();

    fn flatten(self) -> Self::Flattened {}
}

impl<Head, Tail> ToFlat for (Head, Tail)
where
    Tail: ToFlat,
{
    type Flattened = <Tail::Flattened as CanPrepend>::Prepended<Head>;

    fn flatten(self) -> Self::Flattened { self.1.flatten().prepend(self.0) }
}
