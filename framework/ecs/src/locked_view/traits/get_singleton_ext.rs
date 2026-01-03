use std::ops::{Deref, DerefMut};

use crate::{
    locked_view::{LockedView, has_singleton::HasSingleton, locked_view_elements::LockedViewElements},
    traits::singleton::Singleton,
};

mod private {
    pub trait Sealed {}
}

// Extension trait go gain access to a singleton from this view
pub trait LockedViewGetSingletonExt<S: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Gets a component associated with an entity from this view
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<C, S, Idx, QueryIdx> LockedViewGetSingletonExt<S, Idx, QueryIdx> for LockedView<C, S>
where
    S: LockedViewElements,
    C: LockedViewElements,
    Idx: 'static,
{
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>,
    {
        HasSingleton::get_accessor(self).as_ref()
    }
}

pub trait LockedViewGetSingletonMutExt<S: LockedViewElements, Idx>: private::Sealed {
    fn get_singleton_mut<T: Singleton>(&self) -> Option<impl DerefMut<Target = T>>;
}

// impl<C, S, Idx> LockedViewGetSingletonMutExt<S, Idx> for LockedView<C, S>
// where
//     S: LockedViewElements,
//     C: LockedViewElements,
//     Idx: 'static,
// {
// }
