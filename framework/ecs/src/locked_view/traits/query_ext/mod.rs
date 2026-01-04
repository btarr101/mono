use tuples::traits::cons_tuple::ConsTuple;

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView,
        locked_view_elements::LockedViewElements,
        traits::query_ext::locked_view_query::{LockedViewComponentsQuery, LockedViewSingletonsQuery},
    },
};

mod locked_view_query;
mod locked_view_query_element;

// Extension trait used to query a view
pub trait LockedViewQueryExt<C, S, ComponentIdxs, ComponentQueryIdxs, SingletonIdxs, SingletonQueryIdxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
    ComponentIdxs: ConsTuple,
    ComponentQueryIdxs: ConsTuple<Length = ComponentIdxs::Length>,
    SingletonIdxs: ConsTuple,
    SingletonQueryIdxs: ConsTuple<Length = SingletonIdxs::Length>,
{
    /// Queries this view for sets of components and singletons that match the query
    fn query_components_and_singletons<'a, ComponentsQuery, SingletonsQuery>(
        &'a self,
    ) -> impl Iterator<Item = (EntityId, ComponentsQuery::Row, SingletonsQuery::Row)>
    where
        ComponentsQuery: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        SingletonsQuery: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>;

    /// Queries this view for all components in this view
    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, C::Row, S::Row)>
    where
        C: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        S: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>;
}

impl<C, S, ComponentIdxs, ComponentQueryIdxs, SingletonIdxs, SingletonQueryIdxs>
    LockedViewQueryExt<C, S, ComponentIdxs, ComponentQueryIdxs, SingletonIdxs, SingletonQueryIdxs> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    ComponentIdxs: ConsTuple,
    ComponentQueryIdxs: ConsTuple<Length = ComponentIdxs::Length>,
    SingletonIdxs: ConsTuple,
    SingletonQueryIdxs: ConsTuple<Length = SingletonIdxs::Length>,
{
    fn query_components_and_singletons<'a, ComponentsQuery, SingletonsQuery>(
        &'a self,
    ) -> impl Iterator<Item = (EntityId, ComponentsQuery::Row, SingletonsQuery::Row)>
    where
        ComponentsQuery: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        SingletonsQuery: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>,
    {
        // I want to reverse this somehow, but that would require creating an iterator which owns the built singleton row and sounds
        // too much of a PITA rn
        ComponentsQuery::iter_locked_view(self).filter_map(|(entity_id, components)| {
            SingletonsQuery::build_row(self).map(|singletons| (entity_id, components, singletons))
        })
    }

    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, C::Row, S::Row)>
    where
        C: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        S: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>,
    {
        self.query_components_and_singletons::<C, S>()
    }
}

// Extension trait used to query a view
pub trait LockedViewQueryComponentsOrSingletonsExt<C, S, Idxs, QueryIdxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
{
    /// Queries this view for sets of components that match the query
    fn query_components<'a, ComponentsQuery>(&'a self) -> impl Iterator<Item = (EntityId, ComponentsQuery::Row)>
    where
        ComponentsQuery: LockedViewComponentsQuery<'a, C, S, Idxs, QueryIdxs>;

    /// Queries this view for sets of singletons that match the query
    fn query_singletons<'a, SingletonsQuery>(&'a self) -> Option<SingletonsQuery::Row>
    where
        SingletonsQuery: LockedViewSingletonsQuery<'a, C, S, Idxs, QueryIdxs>;
}

impl<C, S, Idxs, QueryIdxs> LockedViewQueryComponentsOrSingletonsExt<C, S, Idxs, QueryIdxs> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
{
    fn query_components<'a, ComponentsQuery>(&'a self) -> impl Iterator<Item = (EntityId, ComponentsQuery::Row)>
    where
        ComponentsQuery: LockedViewComponentsQuery<'a, C, S, Idxs, QueryIdxs>,
    {
        ComponentsQuery::iter_locked_view(self)
    }

    fn query_singletons<'a, SingletonsQuery>(&'a self) -> Option<SingletonsQuery::Row>
    where
        SingletonsQuery: LockedViewSingletonsQuery<'a, C, S, Idxs, QueryIdxs>,
    {
        SingletonsQuery::build_row(self)
    }
}
