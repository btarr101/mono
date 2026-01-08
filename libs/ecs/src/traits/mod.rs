//! Core ECS traits and access abstractions.
//!
//! This module defines the fundamental marker traits and internal access
//! machinery used throughout the ECS.

pub(crate) mod component_set_accessor;
pub(crate) mod component_tuple_element;
pub(crate) mod cons_guards;
pub(crate) mod guard;
pub(crate) mod singleton_container_accessor;
pub(crate) mod singleton_tuple_element;

pub mod singleton {
    /// Marker trait for values stored as ECS singletons.
    pub trait Singleton: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Singleton for T {}
}

pub mod component {
    /// Marker trait for values stored in component sets.
    pub trait Component: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Component for T {}
}
