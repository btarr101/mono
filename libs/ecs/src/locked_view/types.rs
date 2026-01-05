//! Concrete guard tuple aliases for locked views.
//!
//! These type aliases define the concrete cons-style guard layouts used
//! when locking component sets and singleton containers.

use crate::world::{
    component_set::component_set_guards::{ComponentSetReadGuard, ComponentSetWriteGuard},
    singleton_container::singleton_guards::{OptionalSingletonContainerReadGuard, OptionalSingletonContainerWriteGuard},
};

pub type ConsComponentSetGuards<T> = (ComponentSetReadGuard<T>, (ComponentSetWriteGuard<T>, ()));
pub type ConsSingletonContainerGuards<T> = (
    OptionalSingletonContainerReadGuard<T>,
    (OptionalSingletonContainerWriteGuard<T>, ()),
);
