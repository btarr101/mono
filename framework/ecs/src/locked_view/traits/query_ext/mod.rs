use tuples::traits::cons_tuple::ConsTuple;

use crate::{
    entity::EntityId,
    locked_view::{LockedView, locked_view_elements::LockedViewElements, traits::query_ext::locked_view_query::LockedViewQuery},
};

mod locked_view_query;
mod locked_view_query_element;

// Extension trait used to query a view
pub trait LockedViewComponentsQueryExt<C, S, Idxs, QueryIdxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
{
    /// Queries this view for sets of components that match the query
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::Row)>
    where
        Q: LockedViewQuery<'a, C, S, Idxs, QueryIdxs>;

    /// Queries this view for all components in this view
    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, C::Row)>
    where
        C: LockedViewQuery<'a, C, S, Idxs, QueryIdxs>;
}

impl<C, S, Idxs, QueryIdxs> LockedViewComponentsQueryExt<C, S, Idxs, QueryIdxs> for LockedView<C, S>
where
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
    C: LockedViewElements,
    S: LockedViewElements,
{
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::Row)>
    where
        Q: LockedViewQuery<'a, C, S, Idxs, QueryIdxs>,
    {
        Q::iter_locked_view(self)
    }

    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, <C>::Row)>
    where
        C: LockedViewQuery<'a, C, S, Idxs, QueryIdxs>,
    {
        C::iter_locked_view(self)
    }
}
