use itertools::Either;
use tuples::traits::cons_tuple::ConsTuple;

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView,
        locked_view_elements::LockedViewElements,
        traits::query_ext::locked_view_query::{LockedViewComponentsQuery, LockedViewSingletonsQuery},
    },
};

// TODO: unpub
pub mod locked_view_query;
pub mod locked_view_query_element;

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
    /// Queries this view for sets of components that match the query
    fn query<'a, ComponentsQuery, SingletonsQuery>(
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
    fn query<'a, ComponentsQuery, SingletonsQuery>(
        &'a self,
    ) -> impl Iterator<Item = (EntityId, ComponentsQuery::Row, SingletonsQuery::Row)>
    where
        ComponentsQuery: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        SingletonsQuery: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>,
    {
        match SingletonsQuery::build_row(self) {
            None => Either::Left(std::iter::empty()),
            Some(singletons_row) => {
                let singletons = std::iter::repeat(singletons_row);
                Either::Right(
                    ComponentsQuery::iter_locked_view(self)
                        .zip(singletons)
                        .map(|((entity_id, components), singletons)| (entity_id, components, singletons)),
                )
            }
        }
    }

    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, C::Row, S::Row)>
    where
        C: LockedViewComponentsQuery<'a, C, S, ComponentIdxs, ComponentQueryIdxs>,
        S: LockedViewSingletonsQuery<'a, C, S, SingletonIdxs, SingletonQueryIdxs>,
    {
        self.query::<C, S>()
    }
}
