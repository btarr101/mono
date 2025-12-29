use std::ops::{Deref, DerefMut};

use itertools::EitherOrBoth;
use tuples::{
    indexes::{Here, There},
    traits::{as_cons_tuple::AsConsTuple, cons_tuple::ConsTuple, has::ConsHas, has_one_of::ConsHasOne},
};

use crate::{
    component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    entity::EntityId,
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
        component_tuple_element::ComponentTupleElement,
        cons_component_set_guards::{ConsAsComponentSetGuards, ConsComponentSetGuards},
    },
    world::World,
};

mod sealed {
    pub trait Sealed {}
}

/// A view across the world that have certain sets of components and singletons
/// locked accordingly
pub struct LockedView<Elements>
where
    Elements: LockedViewElements,
{
    components: Elements::ConsComponentSetGuards,
}

impl<Elements> LockedView<Elements>
where
    Elements: LockedViewElements,
{
    /// Creates a new locked view
    pub fn new(world: &World) -> Self {
        Self {
            components: Elements::lock_component_sets(world),
        }
    }
}

/// Elements that are used to identify a locked view.
pub trait LockedViewElements {
    type ConsComponentSetGuards: ConsComponentSetGuards;

    /// Gets a cons style tuple of all the guards of component sets in the world
    fn lock_component_sets(world: &World) -> Self::ConsComponentSetGuards;
}

impl<T> LockedViewElements for T
where
    Self: AsConsTuple,
    <Self as AsConsTuple>::As: ConsAsComponentSetGuards,
{
    type ConsComponentSetGuards = <<Self as AsConsTuple>::As as ConsAsComponentSetGuards>::As;

    fn lock_component_sets(world: &World) -> Self::ConsComponentSetGuards {
        Self::ConsComponentSetGuards::cons_lock_from_world(world)
    }
}

/// Extension trait to gain access to components
pub trait LockedViewComponentsExt<Elements: LockedViewElements, Idx, QueryIdx>: sealed::Sealed {
    /// Gets an accessor to the component set in this locked view
    fn components<T: Component>(&self) -> impl ComponentSetAccessor<T>
    where
        Self: HasComponentAccessor<T, Elements, Idx, QueryIdx>;
}

impl<Elements: LockedViewElements> sealed::Sealed for LockedView<Elements> {}
impl<Elements: LockedViewElements, Idx: 'static, QueryIdx: 'static> LockedViewComponentsExt<Elements, Idx, QueryIdx>
    for LockedView<Elements>
{
    fn components<T: Component>(&self) -> impl ComponentSetAccessor<T>
    where
        Self: HasComponentAccessor<T, Elements, Idx, QueryIdx>,
    {
        self.get_accessor()
    }
}

/// Utility trait to determine if the locked view has the component set accessor
pub trait HasComponentAccessor<T: Component, Elements: LockedViewElements, Idx, QueryIdx> {
    type Accessor<'a>: ComponentSetAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_>;
}

type Guards<T> = (ComponentSetReadGuard<T>, (ComponentSetWriteGuard<T>, ()));
impl<T, Elements: LockedViewElements, Idx, QueryIdx> HasComponentAccessor<T, Elements, Idx, QueryIdx> for LockedView<Elements>
where
    T: Component,
    Elements::ConsComponentSetGuards: ConsHasOne<Guards<T>, QueryIdx, Idx>,
    <Elements::ConsComponentSetGuards as ConsHasOne<Guards<T>, QueryIdx, Idx>>::Has: ComponentSetAccessor<T> + 'static,
{
    type Accessor<'a>
        = impl ComponentSetAccessor<T> + 'a
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> { self.components.cons_get_one_ref() }
}

/// Extension trait to gain mutable access to components
pub trait LockedViewComponentsMutExt<Elements: LockedViewElements, Idx>: sealed::Sealed {
    /// Gets an accessor to the component set with mutable components in this locked view
    fn components_mut<T: Component>(&self) -> impl ComponentSetMutAccessor<T>
    where
        Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>;

    /// Gets a fully mutable accessor to the component set in this locked view
    fn mut_components_mut<T: Component>(&mut self) -> impl MutComponentSetMutAccessor<T>
    where
        Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>;
}

impl<Elements: LockedViewElements, Idx> LockedViewComponentsMutExt<Elements, Idx> for LockedView<Elements> {
    fn components_mut<T: Component>(&self) -> impl ComponentSetMutAccessor<T>
    where
        Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
    {
        self.components.cons_get_ref()
    }

    fn mut_components_mut<T: Component>(&mut self) -> impl MutComponentSetMutAccessor<T>
    where
        Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
    {
        self.components.cons_get_mut()
    }
}

// TEMP
pub trait LockedViewComponentsIterExt<Elements: LockedViewElements, Idx, QueryIdx> {
    fn iter<'a, Q: QueryElement<'a, Elements, Idx, QueryIdx>>(&'a self)
    -> impl Iterator<Item = (EntityId, Q::BorrowedComponent)>;
}

impl<Elements, Idx, QueryIdx> LockedViewComponentsIterExt<Elements, Idx, QueryIdx> for LockedView<Elements>
where
    Elements: LockedViewElements,
{
    fn iter<'a, Q: QueryElement<'a, Elements, Idx, QueryIdx>>(
        &'a self,
    ) -> impl Iterator<Item = (EntityId, Q::BorrowedComponent)> {
        Q::iter_locked_view(self)
    }
}
// ======

/// Extension trait used to query a view
pub trait LockedViewComponentsQueryExt<Elements: LockedViewElements, Idx, QueryIdx, TailIdxs, TailQueryIdxs>
where
    TailIdxs: ConsTuple,
    TailQueryIdxs: ConsTuple<Length = TailIdxs::Length>,
{
    /// Queries this view for sets of components that match the query
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::ConsRow)>
    where
        Q: ConsTuple<Length = There<TailIdxs::Length>>,
        Q: ConsQuery<'a, Elements, Idx, QueryIdx, TailIdxs, TailQueryIdxs>;
}

impl<Elements, Idx, QueryIdx, TailIdxs, TailQueryIdxs>
    LockedViewComponentsQueryExt<Elements, Idx, QueryIdx, TailIdxs, TailQueryIdxs> for LockedView<Elements>
where
    TailIdxs: ConsTuple,
    TailQueryIdxs: ConsTuple<Length = TailIdxs::Length>,
    Elements: LockedViewElements,
{
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::ConsRow)>
    where
        Q: ConsTuple<Length = There<TailIdxs::Length>>,
        Q: ConsQuery<'a, Elements, Idx, QueryIdx, TailIdxs, TailQueryIdxs>,
    {
        Q::iter_locked_view(self)
    }
}

/// An element used in a query tuple for a locked view query
pub trait QueryElement<'a, Elements: LockedViewElements, Idx, QueryIdx>: ComponentTupleElement {
    /// The accessors that can be used to iterate over components for this elements
    type Accessors;

    /// The type of the borrow for the component (which depends on what accessor was used)
    type BorrowedComponent;

    /// Gets the correct accessor for a component set from this locked view and iterates across it
    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a;
}

impl<'a, Elements: LockedViewElements + 'a, Idx, QueryIdx, T: Component> QueryElement<'a, Elements, Idx, QueryIdx> for &'a T
where
    Idx: 'static,
    QueryIdx: 'static,
    LockedView<Elements>: HasComponentAccessor<Self::Component, Elements, Idx, QueryIdx>,
{
    type Accessors = Guards<T>;
    type BorrowedComponent = impl Deref<Target = T>;

    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        view.get_accessor().iter()
    }
}

impl<'a, Elements: LockedViewElements + 'a, Idx: 'static, T: Component> QueryElement<'a, Elements, Idx, Here> for &mut T
where
    Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
{
    type Accessors = Guards<T>;
    type BorrowedComponent = impl DerefMut<Target = T>;

    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
        view.components.cons_get_ref().iter_mut()
    }
}

/// A type that can be used to execute a query
pub trait ConsQuery<'a, Elements: LockedViewElements, Idx, QueryIdx, TailIdxs, TailQueryIdxs>
where
    TailIdxs: ConsTuple,
    TailQueryIdxs: ConsTuple<Length = TailIdxs::Length>,
    Self: ConsTuple<Length = There<TailIdxs::Length>>,
{
    type ConsRow;

    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)>;
}

impl<'a, Elements: LockedViewElements, Idx, QueryIdx, Head> ConsQuery<'a, Elements, Idx, QueryIdx, (), ()> for (Head, ())
where
    Head: QueryElement<'a, Elements, Idx, QueryIdx>,
{
    type ConsRow = (Head::BorrowedComponent, ());

    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
        Head::iter_locked_view(view).map(|(entity_id, component)| (entity_id, (component, ())))
    }
}

impl<'a, Elements: LockedViewElements, Idx, QueryIdx, TailIdxs, TailQueryIdxs, Head, Second, Tail>
    ConsQuery<'a, Elements, Idx, QueryIdx, TailIdxs, TailQueryIdxs> for (Head, (Second, Tail))
where
    TailIdxs: ConsTuple,
    TailQueryIdxs: ConsTuple<Length = TailIdxs::Length>,
    Self: ConsTuple<Length = There<TailIdxs::Length>>,
    Head: QueryElement<'a, Elements, Idx, QueryIdx>,
    TailIdxs::Tail: ConsTuple,
    TailQueryIdxs::Tail: ConsTuple<Length = <TailIdxs::Tail as ConsTuple>::Length>,
    (Second, Tail): ConsQuery<'a, Elements, TailIdxs::Head, TailQueryIdxs::Head, TailIdxs::Tail, TailQueryIdxs::Tail>,
{
    type ConsRow =
        (
            Head::BorrowedComponent,
            <(Second, Tail) as ConsQuery<
                'a,
                Elements,
                TailIdxs::Head,
                TailQueryIdxs::Head,
                TailIdxs::Tail,
                TailQueryIdxs::Tail,
            >>::ConsRow,
        );

    fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
        let head = Head::iter_locked_view(view);

        let tail = <(Second, Tail) as ConsQuery<
            'a,
            Elements,
            TailIdxs::Head,
            TailQueryIdxs::Head,
            TailIdxs::Tail,
            TailQueryIdxs::Tail,
        >>::iter_locked_view(view);

        itertools::merge_join_by(head, tail, |(left, _), (right, _)| left.index.cmp(&right.index)).filter_map(|eob| match eob {
            EitherOrBoth::Both((id, left), (_, right)) => Some((id, (left, right))),
            _ => None,
        })
    }
}
