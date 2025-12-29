use crate::{
    component_set_guards::{ComponentSetGuard, ComponentSetReadGuard, ComponentSetWriteGuard},
    traits::component::Component,
};

mod sealed {
    pub trait Sealed {}
}

/// An element used as a specifier in a locked view
pub trait ComponentTupleElement: sealed::Sealed {
    type Component: Component;
    type Guard: ComponentSetGuard;
}
impl<T: Component> sealed::Sealed for &T {}
impl<T: Component> ComponentTupleElement for &T {
    type Component = T;
    type Guard = ComponentSetReadGuard<T>;
}
impl<T: Component> sealed::Sealed for &mut T {}
impl<T: Component> ComponentTupleElement for &mut T {
    type Component = T;
    type Guard = ComponentSetWriteGuard<T>;
}
