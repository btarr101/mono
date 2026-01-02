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
        cons_guards::{ConsAsGuards, ConsGuards},
        singleton::Singleton,
    },
    world::World,
};

/// A view across the world that have certain sets of components and singletons
/// locked accordingly
pub struct LockedView<C, S>
where
    C: private::LockedViewElements,
    S: private::LockedViewElements,
{
    entities: Arc<RwLock<EntityIdAllocator>>,
    components: C::Guards,
    singletons: S::Guards,
}

impl<C, S> LockedView<C, S>
where
    C: private::LockedViewElements,
    S: private::LockedViewElements,
{
    /// Creates a new locked view
    pub fn new(world: &World) -> Self {
        Self {
            entities: world.entities.clone(),
            components: C::lock_from_world(world),
            singletons: S::lock_from_world(world),
        }
    }

    /// Creates a new entity
    pub fn create_entity(&mut self) -> LockedViewEntity<'_, &mut Self> {
        let id = { self.entities.write().allocate_id() };
        LockedViewEntity::new(id, self)
    }

    /// Gets an entity
    ///
    /// You can still mutate components, just cannot add or remove them
    pub fn get_entity(&self, id: EntityId) -> Option<LockedViewEntity<'_, &Self>> {
        { self.entities.read().index_in_use(id.index) }.then_some(LockedViewEntity::new(id, self))
    }

    /// Gets an entity mutably (allows removing and adding components)
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<LockedViewEntity<'_, &mut Self>> {
        { self.entities.read().index_in_use(id.index) }.then_some(LockedViewEntity::new(id, self))
    }
}

/// Extension trait go gain access to a component from this view
pub trait LockedViewGetComponentExt<C: private::LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Gets a component associated with an entity from this view
    fn get_component<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: private::HasComponents<T, C, Idx, QueryIdx>;
}

/// Extension trait go gain access to a component mutably from this view
pub trait LockedViewGetComponentMutExt<C: private::LockedViewElements, Idx>: private::Sealed {
    /// Gets a component associated with an entity mutably from this view
    fn get_component_mut<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: private::HasComponentsMut<T, C, Idx>;

    /// Attempts to add a component to an entity in this view
    ///
    /// Marked as must use, as checking the operation was successful is as simple an ensuring the option is some
    #[must_use]
    fn add_component<T: Component>(&mut self, id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: private::HasComponentsMut<T, C, Idx>;

    /// Attempts to a remove component from an entity,
    /// if a component is removed this way returns it
    fn pop_component<T: Component>(&mut self, id: EntityId) -> Option<T>
    where
        Self: private::HasComponentsMut<T, C, Idx>;
}

// Extension trait go gain access to a singleton from this view
// pub trait LockedViewGetSingletonExt<S: private::LockedViewElements, Idx, QueryIdx>: private::Sealed {
//     /// Gets a component associated with an entity from this view
//     fn get_singleton<T: Singleton>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
//     where
//         Self: private::HasSingleton<T, S, Idx, QueryIdx>;
// }

// Extension trait used to query a view
pub trait LockedViewComponentsQueryExt<C, S, Idxs, QueryIdxs>
where
    C: private::LockedViewElements,
    S: private::LockedViewElements,
    Idxs: ConsTuple,
    QueryIdxs: ConsTuple<Length = Idxs::Length>,
{
    /// Queries this view for sets of components that match the query
    fn query<'a, Q>(&'a self) -> impl Iterator<Item = (EntityId, Q::Row)>
    where
        Q: private::LockedViewQuery<'a, C, S, Idxs, QueryIdxs>;

    /// Queries this view for all components in this view
    fn default_query<'a>(&'a self) -> impl Iterator<Item = (EntityId, C::Row)>
    where
        C: private::LockedViewQuery<'a, C, S, Idxs, QueryIdxs>;
}

pub(crate) mod private {
    use smallvec::SmallVec;

    use super::*;
    use crate::traits::guard::ConsMaybeLockedGuardsExt;

    pub trait Sealed {}
    impl<C: LockedViewElements, S: LockedViewElements> Sealed for LockedView<C, S> {}

    impl<C, S, Idx, QueryIdx> LockedViewGetComponentExt<C, Idx, QueryIdx> for LockedView<C, S>
    where
        C: LockedViewElements,
        S: LockedViewElements,
        Idx: 'static,
        QueryIdx: 'static,
    {
        fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
        where
            Self: HasComponents<T, C, Idx, QueryIdx>,
        {
            self.get_accessor().get(entity_id)
        }
    }

    impl<C, S, Idx> LockedViewGetComponentMutExt<C, Idx> for LockedView<C, S>
    where
        C: LockedViewElements,
        S: LockedViewElements,
        Idx: 'static,
    {
        fn get_component_mut<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
        where
            Self: private::HasComponentsMut<T, C, Idx>,
        {
            self.get_accessor().get(entity_id)
        }

        fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
        where
            Self: private::HasComponentsMut<T, C, Idx>,
        {
            self.get_mut_accessor().try_add(entity_id, component)
        }

        fn pop_component<T: Component>(&mut self, entity_id: EntityId) -> Option<T>
        where
            Self: private::HasComponentsMut<T, C, Idx>,
        {
            self.get_mut_accessor().soft_pop(entity_id)
        }
    }

    // impl<C, S, Idx, QueryIdx> LockedViewGetSingletonExt<S, Idx, QueryIdx> for LockedView<C, S>
    // where
    //     S: LockedViewElements,
    //     C: LockedViewElements,
    //     Idx: 'static,
    // {
    //     fn get_singleton<T: Singleton>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    //     where
    //         Self: private::HasSingleton<T, S, Idx, QueryIdx>,
    //     {
    //         // let x = HasSingleton::get_singleton(self);
    //     }
    // }

    /// C that are used to identify a locked view.
    pub trait LockedViewElements {
        type Guards: ConsGuards;

        /// Gets a cons style tuple of all the guards of component sets in the world
        fn lock_from_world(world: &World) -> Self::Guards;
    }

    impl<T> LockedViewElements for T
    where
        Self: AsConsTuple,
        <Self as AsConsTuple>::As: ConsAsGuards,
    {
        type Guards = <<Self as AsConsTuple>::As as ConsAsGuards>::As;

        fn lock_from_world(world: &World) -> Self::Guards {
            // First, get all the locks in "maybe locked" form. These will be ordered the same as C
            let mut maybe_locks = Self::Guards::get_maybe_locks(world);

            // Collect dyn references to all of the maybe locks in a Vec (todo: use smallvec to keep this on the stack)
            let mut dyn_maybe_locks = maybe_locks.dyn_muts().collect::<SmallVec<[_; 8]>>();

            // !! Important !!
            // We sort the dyn locks by the type id of the component. That way we have a stable lock order to prevent deadlocks,
            // then lock them
            dyn_maybe_locks.sort_by_key(|dyn_lock| dyn_lock.element_type_id());
            dyn_maybe_locks.into_iter().for_each(|dyn_lock| dyn_lock.lock());

            // Finally we convert the locks back to there locked guards
            maybe_locks.to_locked_guards()
        }
    }

    /// Utility trait to determine if the locked view has the component set accessor
    pub trait HasComponents<T: Component, C: LockedViewElements, Idx, QueryIdx>: Sealed {
        type Accessor<'a>: ComponentSetAccessor<T>
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_>;
    }

    type Guards<T> = (ComponentSetReadGuard<T>, (ComponentSetWriteGuard<T>, ()));
    impl<T, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx> HasComponents<T, C, Idx, QueryIdx> for LockedView<C, S>
    where
        T: Component,
        C::Guards: ConsHasOne<Guards<T>, QueryIdx, Idx>,
        <C::Guards as ConsHasOne<Guards<T>, QueryIdx, Idx>>::Has: ComponentSetAccessor<T> + 'static,
    {
        type Accessor<'a>
            = impl ComponentSetAccessor<T> + 'a
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_> { self.components.cons_get_one_ref() }
    }

    /// Utility trait to determine if the locked view has a mutable component set accessor
    pub trait HasComponentsMut<T: Component, C: LockedViewElements, Idx>: Sealed {
        type Accessor<'a>: ComponentSetMutAccessor<T>
        where
            Self: 'a;
        type MutAccessor<'a>: MutComponentSetMutAccessor<T>
        where
            Self: 'a;

        fn get_accessor(&self) -> &Self::Accessor<'_>;
        fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_>;
    }

    impl<T: Component, C: LockedViewElements, S: LockedViewElements, Idx> HasComponentsMut<T, C, Idx> for LockedView<C, S>
    where
        C::Guards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
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

    // GOING TO TAKE A PAUSE ON THIS, NEED TO PUT THE SINGLETON INTO A DANGER CELL!!!
    // TODO: For 2morrow
    // pub trait HasSingleton<T: Singleton, S: LockedViewElements, Idx, QueryIdx>: Sealed {
    //     type Accessor<'a>

    //     fn get_singleton(&self) -> &impl Deref<Target = T>;
    // }

    // impl<T, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx> HasSingleton<T, S, Idx, QueryIdx> for LockedView<C, S>
    // where
    //     T: Singleton,
    //     S::Guards: ConsHasOne<Guards<T>, QueryIdx, Idx>,
    //     for<'a> <S::Guards as ConsHasOne<Guards<T>, QueryIdx, Idx>>::Has: Deref<Target = T> + 'a,
    // {
    //     fn get_singleton(&self) -> &impl Deref<Target = T> { self.singletons.cons_get_one_ref() }
    // }

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
            C: private::LockedViewQuery<'a, C, S, Idxs, QueryIdxs>,
        {
            C::iter_locked_view(self)
        }
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
        type Row = <<<Self as AsConsTuple>::As as LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>>::ConsRow as Flat>::Flattened;

        fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::Row)> {
            <<Self as AsConsTuple>::As as LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>>::iter_locked_view(view)
                .map(|(entity_id, components)| (entity_id, components.flatten()))
        }
    }

    /// An element used in a query tuple for a locked view query
    pub trait LockedViewQueryElement<'a, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx>:
        ComponentTupleElement
    {
        /// The accessors that can be used to iterate over components for this C
        type Accessors;

        /// The type of the borrow for the component (which depends on what accessor was used)
        type BorrowedComponent;

        /// Gets the correct accessor for a component set from this locked view and iterates across it
        fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a;
    }

    impl<'a, C, S, Idx, QueryIdx, T: Component> LockedViewQueryElement<'a, C, S, Idx, QueryIdx> for &'a T
    where
        C: LockedViewElements + 'a,
        S: LockedViewElements + 'a,
        Idx: 'static,
        QueryIdx: 'static,
        LockedView<C, S>: HasComponents<Self::Component, C, Idx, QueryIdx>,
    {
        type Accessors = Guards<T>;
        type BorrowedComponent = impl Deref<Target = T>;

        fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
            view.get_accessor().iter()
        }
    }

    impl<'a, C, S, Idx, T: Component> LockedViewQueryElement<'a, C, S, Idx, Here> for &mut T
    where
        C: LockedViewElements + 'a,
        C::Guards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
        S: LockedViewElements + 'a,
        Idx: 'static,
    {
        type Accessors = Guards<T>;
        type BorrowedComponent = impl DerefMut<Target = T>;

        fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::BorrowedComponent)> + 'a {
            view.components.cons_get_ref().iter_mut()
        }
    }

    /// A type that can be used to execute a query
    pub trait LockedViewConsQuery<'a, C, S, Idxs, QueryIdxs>: Sealed
    where
        C: LockedViewElements,
        S: LockedViewElements,
        Self: ConsTuple,
        Idxs: ConsTuple<Length = Self::Length>,
        QueryIdxs: ConsTuple<Length = Self::Length>,
    {
        type ConsRow: Flat;

        fn iter_locked_view(view: &'a LockedView<C, S>) -> impl Iterator<Item = (EntityId, Self::ConsRow)>;
    }

    impl<Head> Sealed for (Head, ()) {}
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

    impl<Head, Second, Tail> Sealed for (Head, (Second, Tail)) {}
    impl<'a, C, S, Idx, QueryIdx, TailIdxs, TailQueryIdxs, Head, Tail>
        LockedViewConsQuery<'a, C, S, (Idx, TailIdxs), (QueryIdx, TailQueryIdxs)> for (Head, Tail)
    where
        C: LockedViewElements,
        S: LockedViewElements,
        Self: Sealed,
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

            itertools::merge_join_by(head, tail, |(left, _), (right, _)| left.index.cmp(&right.index)).filter_map(|eob| match eob
            {
                EitherOrBoth::Both((id, left), (_, right)) => Some((id, (left, right))),
                _ => None,
            })
        }
    }
}
