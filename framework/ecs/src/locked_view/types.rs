use crate::world::{
    component_set::component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    singleton_guards::{OptionalSingletonContainerReadGuard, OptionalSingletonContainerWriteGuard},
};

pub type ConsComponentSetGuards<T> = (ComponentSetReadGuard<T>, (ComponentSetWriteGuard<T>, ()));
pub type ConsSingletonContainerGuards<T> = (
    OptionalSingletonContainerReadGuard<T>,
    (OptionalSingletonContainerWriteGuard<T>, ()),
);
