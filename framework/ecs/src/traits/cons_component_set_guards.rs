use crate::{component_set_guards::ComponentSetGuard, traits::component_tuple_element::ComponentTupleElement, world::World};

mod sealed {
    pub trait Sealed {}
}

/// Trait for a tuple set of component sets
pub trait ConsComponentSetGuards: sealed::Sealed {
    /// Gets this component set from the world
    fn cons_lock_from_world(world: &World) -> Self;
}
/// Util trait to convert a cons tuple of components to component sets
pub trait ConsAsComponentSetGuards: sealed::Sealed {
    type As: ConsComponentSetGuards;
}

impl sealed::Sealed for () {}
impl ConsComponentSetGuards for () {
    fn cons_lock_from_world(_: &World) -> Self {}
}
impl ConsAsComponentSetGuards for () {
    type As = ();
}

impl<Head, Tail> sealed::Sealed for (Head, Tail) {}
impl<Head, Tail> ConsComponentSetGuards for (Head, Tail)
where
    Head: ComponentSetGuard,
    Tail: ConsComponentSetGuards,
{
    fn cons_lock_from_world(world: &World) -> Self { (Head::from_world(world), Tail::cons_lock_from_world(world)) }
}
impl<Head, Tail> ConsAsComponentSetGuards for (Head, Tail)
where
    Head: ComponentTupleElement,
    Tail: ConsAsComponentSetGuards,
{
    type As = (Head::Guard, Tail::As);
}
