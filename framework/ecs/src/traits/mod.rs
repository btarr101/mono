pub(crate) mod component_set_accessor;
pub(crate) mod component_tuple_element;
pub(crate) mod cons_component_set_guards;

pub mod singleton {
    pub trait Singleton: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Singleton for T {}
}

pub mod component {
    pub trait Component: 'static + Send + Sync {}
    impl<T: 'static + Send + Sync> Component for T {}
}
