use std::ops::{Deref, DerefMut};

use crate::sparse_set::SparseSet;

pub mod component_set_accessor;
pub mod component_tuple_element;
pub mod cons_component_set_guards;

pub mod singleton {
    use super::*;

    pub trait Singleton: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Singleton for T {}

    pub trait HasSingleton<T: Singleton> {
        fn singleton(&self) -> impl Deref<Target = Option<T>>;
    }

    pub trait HasSingletonMut<T: Singleton> {
        fn singleton_mut(&self) -> impl DerefMut<Target = Option<T>>;
    }
}

pub mod component {
    use super::*;

    pub trait Component: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Component for T {}

    pub trait HasComponents<T: Component> {
        fn components(&self) -> impl Deref<Target = SparseSet<T>>;
    }

    pub trait HasComponentsMut<T: Component> {
        fn components_mut(&self) -> impl DerefMut<Target = SparseSet<T>>;
    }
}
