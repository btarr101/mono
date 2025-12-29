use crate::traits::has::ConsHas;

/// Trait used to determine if a tuple of types is a superset of another tuple with types
pub trait ConsSuperSet<Subset, Idxs> {
    /// Gets the cons tuple subset of this tuple
    fn cons_subset(self) -> Subset
    where
        Self: Copy;
}

impl<T> ConsSuperSet<(), ()> for T {
    fn cons_subset(self)
    where
        Self: Copy,
    {
    }
}

impl<Head, Tail, Tuple, Idx, TailIdxs> ConsSuperSet<(Head, Tail), (Idx, TailIdxs)> for Tuple
where
    Self: ConsHas<Head, Idx> + ConsSuperSet<Tail, TailIdxs>,
{
    fn cons_subset(self) -> (Head, Tail)
    where
        Self: Copy,
    {
        (self.cons_get(), self.cons_subset())
    }
}
