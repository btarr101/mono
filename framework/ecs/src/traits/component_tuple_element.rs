use crate::{
    traits::{component::Component, guard::Guard},
    world::component_set::component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
};

mod private {
    pub trait Sealed {}
}

/// An element used as a specifier in a locked view
pub trait ComponentTupleElement: private::Sealed {
    type Component: Component;
    type Guard: Guard<Element = Self::Component>;
}

impl<T: Component> private::Sealed for &T {}
impl<T: Component> ComponentTupleElement for &T {
    type Component = T;
    type Guard = ComponentSetReadGuard<T>;
}

impl<T: Component> private::Sealed for &mut T {}
impl<T: Component> ComponentTupleElement for &mut T {
    type Component = T;
    type Guard = ComponentSetWriteGuard<T>;
}
