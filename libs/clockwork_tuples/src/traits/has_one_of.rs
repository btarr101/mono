use crate::{
    index::{Here, Index, There},
    traits::has::ConsHas,
};

/// Selects a single matching element from a cons-style tuple.
///
/// # Invariants
/// - Exactly one element satisfies the query.
/// - Selection order follows tuple order.
pub trait ConsHasOne<Query, QueryIdx, Idx = Here> {
    /// Selected element type.
    type Has;

    /// Gets the first element out of the query
    fn cons_get_one(self) -> Self::Has;

    /// Gets the first element out of the query
    fn cons_get_one_ref(&self) -> &Self::Has;
}

impl<Query, QueryIdx, Head, Tail> ConsHasOne<Query, QueryIdx, Here> for (Head, Tail)
where
    Query: ConsHas<Head, QueryIdx>,
{
    type Has = Head;

    fn cons_get_one(self) -> Self::Has { self.0 }
    fn cons_get_one_ref(&self) -> &Self::Has { &self.0 }
}

impl<Query, QueryIdx, Head, Tail, TailIdx: Index> ConsHasOne<Query, QueryIdx, There<TailIdx>> for (Head, Tail)
where
    Tail: ConsHasOne<Query, QueryIdx, TailIdx>,
{
    type Has = <Tail as ConsHasOne<Query, QueryIdx, TailIdx>>::Has;

    fn cons_get_one(self) -> Self::Has { self.1.cons_get_one() }
    fn cons_get_one_ref(&self) -> &Self::Has { self.1.cons_get_one_ref() }
}
