use itertools::EitherOrBoth;
use tuples::{
    index::There,
    traits::{as_cons_tuple::AsConsTuple, cons_tuple::ConsTuple, flat::ToFlat},
};

use crate::{
    entity::EntityId,
    locked_view::{
        LockedView, locked_view_elements::LockedViewElements,
        traits::query_ext::locked_view_query_element::LockedViewQueryElement,
    },
};

mod private {
    pub trait Sealed {}
}

/// Trait for what can be used as a query over a locked view
pub trait LockedViewQuery<'a, C, S, Idxs, QueryIdxs>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    type Row;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::Row)>;
}

impl<'a, C, S, Idxs, QueryIdxs, Tuple> LockedViewQuery<'a, C, S, Idxs, QueryIdxs> for Tuple
where
    C: LockedViewElements,
    S: LockedViewElements,
    Self: AsConsTuple,
    <Self as AsConsTuple>::As: ConsTuple,
    Idxs: ConsTuple<Length = <<Self as AsConsTuple>::As as ConsTuple>::Length>,
    QueryIdxs: ConsTuple<Length = <<Self as AsConsTuple>::As as ConsTuple>::Length>,
    <Self as AsConsTuple>::As: LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>,
{
    type Row = <<<Self as AsConsTuple>::As as LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>>::ConsRow as ToFlat>::Flattened;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::Row)> {
        <<Self as AsConsTuple>::As as LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>>::iter_locked_view(view)
            .map(|(entity_id, components)| (entity_id, components.flatten()))
    }
}

/// A type that can be used to execute a query
pub trait LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>: private::Sealed
where
    C: LockedViewElements,
    S: LockedViewElements,
    Self: ConsTuple,
    Idxs: ConsTuple<Length = Self::Length>,
    QueryIdxs: ConsTuple<Length = Self::Length>,
{
    type ConsRow: ToFlat;

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::ConsRow)>;
}

impl<Head> private::Sealed for (Head, ()) {}
impl<'a, C, S, Idx, QueryIdx, Head> LockedViewConsQuery<'a, C, S, (Idx, ()), (QueryIdx, ())> for (Head, ())
where
    C: LockedViewElements,
    S: LockedViewElements,
    Head: LockedViewQueryElement<'a, C, S, Idx, QueryIdx>,
{
    type ConsRow = (Head::BorrowedComponent, ());

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
        Head::iter_locked_view(view).map(|(entity_id, component)| (entity_id, (component, ())))
    }
}

impl<Head, Second, Tail> private::Sealed for (Head, (Second, Tail)) {}
impl<'a, C, S, Idx, QueryIdx, TailIdxs, TailQueryIdxs, Head, Tail>
    LockedViewConsQuery<'a, C, S, (Idx, TailIdxs), (QueryIdx, TailQueryIdxs)> for (Head, Tail)
where
    C: LockedViewElements,
    S: LockedViewElements,
    Self: private::Sealed,
    Self: ConsTuple<Length = There<TailIdxs::Length>>,
    Head: LockedViewQueryElement<'a, C, S, Idx, QueryIdx>,
    // Check tail
    Tail: ConsTuple,
    TailIdxs: ConsTuple<Length = Tail::Length>,
    TailQueryIdxs: ConsTuple<Length = Tail::Length>,
    Tail: LockedViewConsQuery<'a, C, S, TailIdxs, TailQueryIdxs>,
{
    type ConsRow = (
        Head::BorrowedComponent,
        <Tail as LockedViewConsQuery<'a, C, S, TailIdxs, TailQueryIdxs>>::ConsRow,
    );

    fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
        let head = Head::iter_locked_view(view);

        let tail = <Tail as LockedViewConsQuery<'a, C, S, TailIdxs, TailQueryIdxs>>::iter_locked_view(view);

        itertools::merge_join_by(head, tail, |(left, _), (right, _)| left.index.cmp(&right.index)).filter_map(|eob| match eob {
            EitherOrBoth::Both((id, left), (_, right)) => Some((id, (left, right))),
            _ => None,
        })
    }
}
