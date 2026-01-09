//! Singleton access traits for locked views.
//!
//! These traits provide compile-time guarantees for accessing singleton
//! containers through a `LockedView`.

use clockwork_tuples::traits::{has::ConsHas, has_one_of::ConsHasOne};

use crate::{
    locked_view::{LockedView, locked_view_elements::LockedViewElements, types::ConsSingletonContainerGuards},
    traits::{
        singleton::Singleton,
        singleton_container_accessor::{
            MutSingletonContainerMutAccessor, SingletonContainerAccessor, SingletonContainerMutAccessor,
        },
    },
    world::singleton_container::singleton_guards::OptionalSingletonContainerWriteGuard,
};

mod private {
    pub trait Sealed {}
}

pub trait HasSingleton<T: Singleton, S: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    type Accessor<'a>: SingletonContainerAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<T, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx> HasSingleton<T, S, Idx, QueryIdx> for LockedView<C, S>
where
    Idx: 'static,
    QueryIdx: 'static,
    T: Singleton,
    S::SingletonContainerGuards: ConsHasOne<ConsSingletonContainerGuards<T>, QueryIdx, Idx>,
    for<'a> <S::SingletonContainerGuards as ConsHasOne<ConsSingletonContainerGuards<T>, QueryIdx, Idx>>::Has:
        SingletonContainerAccessor<T>,
{
    type Accessor<'a>
        = impl SingletonContainerAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> { self.singletons.cons_get_one_ref() }
}

pub trait HasSingletonMut<T: Singleton, S: LockedViewElements, Idx>: private::Sealed {
    type Accessor<'a>: SingletonContainerMutAccessor<T>
    where
        Self: 'a;
    type MutAccessor<'a>: MutSingletonContainerMutAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_>;
    fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_>;
}

impl<T, C: LockedViewElements, S: LockedViewElements, Idx> HasSingletonMut<T, S, Idx> for LockedView<C, S>
where
    Idx: 'static,
    T: Singleton,
    S::SingletonContainerGuards: ConsHas<OptionalSingletonContainerWriteGuard<T>, Idx>,
{
    type Accessor<'a>
        = impl SingletonContainerMutAccessor<T>
    where
        Self: 'a;
    type MutAccessor<'a>
        = impl MutSingletonContainerMutAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> { self.singletons.cons_get_ref() }
    fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_> { self.singletons.cons_get_mut() }
}
