use crate::traits::{cons_tuple::ConsTuple, has::ConsHas};

/// Extracts a cons-style subset from a superset tuple.
///
/// # Invariants
/// - All elements of `Subset` exist in `Self`.
/// - Relative order of extracted elements is preserved.
pub trait ConsSuperSet<Subset, Idxs> {
    /// Gets the cons tuple subset of this tuple
    fn cons_subset(self) -> Subset;
}

impl<T: ConsTuple> ConsSuperSet<(), ()> for T {
    fn cons_subset(self) {}
}

impl<Head, Tail, Tuple, Idx, TailIdxs> ConsSuperSet<(Head, Tail), (Idx, TailIdxs)> for Tuple
where
    Self: ConsHas<Head, Idx>,
    <Self as ConsHas<Head, Idx>>::Plucked: ConsSuperSet<Tail, TailIdxs>,
{
    fn cons_subset(self) -> (Head, Tail) {
        let (head, plucked) = self.cons_pluck();
        (head, plucked.cons_subset())
    }
}
