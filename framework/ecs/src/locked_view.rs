use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use itertools::EitherOrBoth;
use parking_lot::RwLock;
use tuples::{
    indexes::{Here, There},
    traits::{as_cons_tuple::AsConsTuple, cons_tuple::ConsTuple, flat::Flat, has::ConsHas, has_one_of::ConsHasOne},
};

use crate::{
    component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    entity::{EntityId, LockedViewEntity},
    entity_id_allocator::EntityIdAllocator,
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
        component_tuple_element::ComponentTupleElement,
        cons_component_set_guards::{ConsAsComponentSetGuards, ConsComponentSetGuards},
    },
    world::World,
};

/// A view across the world that have certain sets of components and singletons
/// locked accordingly
pub struct LockedView<Elements>
where
    Elements: private::LockedViewElements,
{
    entities: Arc<RwLock<EntityIdAllocator>>,
    components: Elements::ConsComponentSetGuards,
}

impl<Elements> LockedView<Elements>
where
    Elements: private::LockedViewElements,
{
    /// Creates a new locked view
    pub fn new(world: &World) -> Self {
        Self {
            entities: world.entities.clone(),
            components: Elements::lock_component_sets(world),
        }
    }

    /// Creates a new entity
    pub fn create_entity(&mut self) -> LockedViewEntity<'_, Elements> {
        let id = { self.entities.write().allocate_id() };
        LockedViewEntity::new(id, self)
    }

    /// Gets an entity
    pub fn get_entity(&mut self, id: EntityId) -> Option<LockedViewEntity<'_, Elements>> {
        { self.entities.read().index_in_use(id.index) }.then_some(LockedViewEntity::new(id, self))
    }
}

/// Extension trait go gain access to a component from this view
pub trait LockedViewGetComponentExt<Elements: private::LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Gets a component associated with an entity from this view
    fn get_component<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: private::HasComponents<T, Elements, Idx, QueryIdx>;
}

/// Extension trait go gain access to a component mutably from this view
pub trait LockedViewGetComponentMutExt<Elements: private::LockedViewElements, Idx>: private::Sealed {
    /// Gets a component associated with an entity mutably from this view
    fn get_component_mut<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: private::HasComponentsMut<T, Elements, Idx>;

    /// Attempts to add a component to an entity in this view
    ///
    /// Marked as must use, as checking the operation was successful is as simple an ensuring the option is some
    #[must_use]
    fn add_component<T: Component>(&mut self, id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: private::HasComponentsMut<T, Elements, Idx>;

    /// Attempts to a remove component from an entity,
    /// if a component is removed this way returns it
    fn pop_component<T: Component>(&mut self, id: EntityId) -> Option<T>
    where
        Self: private::HasComponentsMut<T, Elements, Idx>;
}

// Extension trait used to query a view
pub trait LockedViewComponentsQueryExt<Elements, Idxs, QueryIdxs>
where
    Elements: private::LockedViewElements,
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
{
    /// Queries this view for sets of components that match the query
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::Row)>
    where
        Q: private::LockedViewQuery<'a, Elements, Idxs, QueryIdxs>;

    /// Queries this view for all components in this view
    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, Elements::Row)>
    where
        Elements: private::LockedViewQuery<'a, Elements, Idxs, QueryIdxs>;
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}
    impl<Elements: LockedViewElements> Sealed for LockedView<Elements> {}

    impl<Elements, Idx, QueryIdx> LockedViewGetComponentExt<Elements, Idx, QueryIdx> for LockedView<Elements>
    where
        Elements: LockedViewElements,
        Idx: 'static,
        QueryIdx: 'static,
    {
        fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
        where
            Self: HasComponents<T, Elements, Idx, QueryIdx>,
        {
            self.get_accessor().get(entity_id)
        }
    }

    impl<Elements, Idx> LockedViewGetComponentMutExt<Elements, Idx> for LockedView<Elements>
    where
        Elements: LockedViewElements,
        Idx: 'static,
    {
        fn get_component_mut<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
        where
            Self: private::HasComponentsMut<T, Elements, Idx>,
        {
            self.get_accessor().get(entity_id)
        }

        fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
        where
            Self: private::HasComponentsMut<T, Elements, Idx>,
        {
            self.get_mut_accessor().try_add(entity_id, component)
        }

        fn pop_component<T: Component>(&mut self, entity_id: EntityId) -> Option<T>
        where
            Self: private::HasComponentsMut<T, Elements, Idx>,
        {
            self.get_mut_accessor().soft_pop(entity_id)
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

    /// Utility trait to determine if the locked view has the component set accessor
    pub trait HasComponents<T: Component, Elements: LockedViewElements, Idx, QueryIdx>: Sealed {
        type Accessor<'a>: ComponentSetAccessor<T>
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_>;
    }

    type Guards<T> = (ComponentSetReadGuard<T>, (ComponentSetWriteGuard<T>, ()));
    impl<T, Elements: LockedViewElements, Idx, QueryIdx> HasComponents<T, Elements, Idx, QueryIdx> for LockedView<Elements>
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

    /// Utility trait to determine if the locked view has a mutable component set accessor
    pub trait HasComponentsMut<T: Component, Elements: LockedViewElements, Idx>: Sealed {
        type Accessor<'a>: ComponentSetMutAccessor<T>
        where
            Self: 'a;
        type MutAccessor<'a>: MutComponentSetMutAccessor<T>
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_>;
        fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_>;
    }

    impl<T: Component, Elements: LockedViewElements, Idx> HasComponentsMut<T, Elements, Idx> for LockedView<Elements>
    where
        Elements::ConsComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
    {
        type Accessor<'a>
            = impl ComponentSetMutAccessor<T> + 'a
        where
            Self: 'a;
        type MutAccessor<'a>
            = impl MutComponentSetMutAccessor<T> + 'a
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_> { self.components.cons_get_ref() }
        fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_> { self.components.cons_get_mut() }
    }

    impl<Elements, Idxs, QueryIdxs> LockedViewComponentsQueryExt<Elements, Idxs, QueryIdxs> for LockedView<Elements>
    where
        Idxs: ConsTuple,
        QueryIdxs: ConsTuple<Length = Idxs::Length>,
        Elements: LockedViewElements,
    {
        fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::Row)>
        where
            Q: LockedViewQuery<'a, Elements, Idxs, QueryIdxs>,
        {
            Q::iter_locked_view(self)
        }

        fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, <Elements>::Row)>
        where
            Elements: private::LockedViewQuery<'a, Elements, Idxs, QueryIdxs>,
        {
            Elements::iter_locked_view(self)
        }
    }

    /// Trait for what can be used as a query over a locked view
    pub trait LockedViewQuery<'a, Elements, Idxs, QueryIdxs>
    where
        Elements: LockedViewElements,
    {
        type Row;

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::Row)>;
    }

    impl<'a, Elements, Idxs, QueryIdxs, Tuple> LockedViewQuery<'a, Elements, Idxs, QueryIdxs> for Tuple
    where
        Elements: LockedViewElements,
        Self: AsConsTuple,
        <Self as AsConsTuple>::As: ConsTuple,
        Idxs: ConsTuple<Length = <<Self as AsConsTuple>::As as ConsTuple>::Length>,
        QueryIdxs: ConsTuple<Length = <<Self as AsConsTuple>::As as ConsTuple>::Length>,
        <Self as AsConsTuple>::As: LockedViewConsQuery<'a, Elements, Idxs, QueryIdxs>,
    {
        type Row =
            <<<Self as AsConsTuple>::As as LockedViewConsQuery<'a, Elements, Idxs, QueryIdxs>>::ConsRow as Flat>::Flattened;

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::Row)> {
            <<Self as AsConsTuple>::As as LockedViewConsQuery<'a, Elements, Idxs, QueryIdxs>>::iter_locked_view(view)
                .map(|(entity_id, components)| (entity_id, components.flatten()))
        }
    }

    /// An element used in a query tuple for a locked view query
    pub trait LockedViewQueryElement<'a, Elements: LockedViewElements, Idx, QueryIdx>: ComponentTupleElement {
        /// The accessors that can be used to iterate over components for this elements
        type Accessors;

        /// The type of the borrow for the component (which depends on what accessor was used)
        type BorrowedComponent;

        /// Gets the correct accessor for a component set from this locked view and iterates across it
        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a;
    }

    impl<'a, Elements: LockedViewElements + 'a, Idx, QueryIdx, T: Component> LockedViewQueryElement<'a, Elements, Idx, QueryIdx>
        for &'a T
    where
        Idx: 'static,
        QueryIdx: 'static,
        LockedView<Elements>: HasComponents<Self::Component, Elements, Idx, QueryIdx>,
    {
        type Accessors = Guards<T>;
        type BorrowedComponent = impl Deref<Target = T>;

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
            view.get_accessor().iter()
        }
    }

    impl<'a, Elements: LockedViewElements + 'a, Idx: 'static, T: Component> LockedViewQueryElement<'a, Elements, Idx, Here> for &mut T
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
    pub trait LockedViewConsQuery<'a, Elements: LockedViewElements, Idxs, QueryIdxs>: Sealed
    where
        Self: ConsTuple,
        Idxs: ConsTuple<Length = Self::Length>,
        QueryIdxs: ConsTuple<Length = Self::Length>,
    {
        type ConsRow: Flat;

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)>;
    }

    impl<Head> Sealed for (Head, ()) {}
    impl<'a, Elements: LockedViewElements, Idx, QueryIdx, Head> LockedViewConsQuery<'a, Elements, (Idx, ()), (QueryIdx, ())>
        for (Head, ())
    where
        Head: LockedViewQueryElement<'a, Elements, Idx, QueryIdx>,
    {
        type ConsRow = (Head::BorrowedComponent, ());

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
            Head::iter_locked_view(view).map(|(entity_id, component)| (entity_id, (component, ())))
        }
    }

    impl<Head, Second, Tail> Sealed for (Head, (Second, Tail)) {}
    impl<'a, Elements: LockedViewElements, Idx, QueryIdx, TailIdxs, TailQueryIdxs, Head, Tail>
        LockedViewConsQuery<'a, Elements, (Idx, TailIdxs), (QueryIdx, TailQueryIdxs)> for (Head, Tail)
    where
        Self: Sealed,
        Self: ConsTuple<Length = There<TailIdxs::Length>>,
        Head: LockedViewQueryElement<'a, Elements, Idx, QueryIdx>,
        // Check tail
        Tail: ConsTuple,
        TailIdxs: ConsTuple<Length = Tail::Length>,
        TailQueryIdxs: ConsTuple<Length = Tail::Length>,
        Tail: LockedViewConsQuery<'a, Elements, TailIdxs, TailQueryIdxs>,
    {
        type ConsRow = (
            Head::BorrowedComponent,
            <Tail as LockedViewConsQuery<'a, Elements, TailIdxs, TailQueryIdxs>>::ConsRow,
        );

        fn iter_locked_view(view: &'a LockedView<Elements>) -> impl Iterator<Item = (EntityId, Self::ConsRow)> {
            let head = Head::iter_locked_view(view);

            let tail = <Tail as LockedViewConsQuery<'a, Elements, TailIdxs, TailQueryIdxs>>::iter_locked_view(view);

            itertools::merge_join_by(head, tail, |(left, _), (right, _)| left.index.cmp(&right.index)).filter_map(|eob| match eob
            {
                EitherOrBoth::Both((id, left), (_, right)) => Some((id, (left, right))),
                _ => None,
            })
        }
    }
}
