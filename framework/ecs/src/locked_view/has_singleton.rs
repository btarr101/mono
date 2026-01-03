use std::ops::{Deref, DerefMut};

use tuples::traits::has_one_of::ConsHasOne;

use crate::{
    locked_view::{LockedView, locked_view_elements::LockedViewElements, types::ConsSingletonContainerGuards},
    traits::singleton::Singleton,
};

mod private {
    pub trait Sealed {}
}

pub trait HasSingleton<T: Singleton, S: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    type Accessor<'a>: Deref<Target = Option<T>> + 'a
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
        Deref<Target = Option<T>> + 'a,
{
    type Accessor<'a>
        = impl Deref<Target = Option<T>>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> { self.singletons.cons_get_one_ref() }
}

pub trait HasSingletonMut<T: Singleton, S: LockedViewElements, Idx>: private::Sealed {
    type Accessor<'a>: DerefMut<Target = Option<T>> + 'a
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_>;
}
