//! Component access traits for locked views.
//!
//! These traits describe compile-time guarantees for accessing component
//! sets through a `LockedView`.

use clockwork_tuples::traits::{has::ConsHas, has_one_of::ConsHasOne};

use crate::{
    locked_view::{LockedView, locked_view_elements::LockedViewElements, types::ConsComponentSetGuards},
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
    },
    world::component_set::component_set_guards::ComponentSetWriteGuard,
};

mod private {
    pub trait Sealed {}
}

/// Utility trait to determine if the locked view has the component set accessor
pub trait HasComponents<T: Component, C: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    type Accessor<'a>: ComponentSetAccessor<T>
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<T, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx> HasComponents<T, C, Idx, QueryIdx> for LockedView<C, S>
where
    T: Component,
    C::ComponentSetGuards: ConsHasOne<ConsComponentSetGuards<T>, QueryIdx, Idx>,
    <C::ComponentSetGuards as ConsHasOne<ConsComponentSetGuards<T>, QueryIdx, Idx>>::Has: ComponentSetAccessor<T> + 'static,
{
    type Accessor<'a>
        = impl ComponentSetAccessor<T> + 'a
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> {
        self.components.cons_get_one_ref()
    }
}

/// Utility trait to determine if the locked view has a mutable component set accessor
pub trait HasComponentsMut<T: Component, C: LockedViewElements, Idx>: private::Sealed {
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
    C::ComponentSetGuards: ConsHas<ComponentSetWriteGuard<T>, Idx>,
{
    type Accessor<'a>
        = impl ComponentSetMutAccessor<T> + 'a
    where
        Self: 'a;
    type MutAccessor<'a>
        = impl MutComponentSetMutAccessor<T> + 'a
    where
        Self: 'a;

    fn get_accessor(&self) -> &Self::Accessor<'_> {
        self.components.cons_get_ref()
    }
    fn get_mut_accessor(&mut self) -> &mut Self::MutAccessor<'_> {
        self.components.cons_get_mut()
    }
}
