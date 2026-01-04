use crate::traits::{cons_tuple::ConsTuple, has::ConsHas};

/// Trait used to determine if a tuple of types is a superset of another tuple with types
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
