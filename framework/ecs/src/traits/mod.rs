pub(crate) mod component_set_accessor;
pub(crate) mod component_tuple_element;
pub(crate) mod cons_guards;
pub(crate) mod guard;
pub(crate) mod singleton_container_accessor;
pub(crate) mod singleton_tuple_element;

pub mod singleton {
    pub trait Singleton: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Singleton for T {}
}

pub mod component {
    pub trait Component: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Component for T {}
}
