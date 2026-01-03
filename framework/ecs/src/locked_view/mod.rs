use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    entity::{EntityId, LockedViewEntity},
    locked_view::locked_view_elements::LockedViewElements,
    world::{World, entity_id_allocator::EntityIdAllocator},
};

pub(crate) mod has_components;
pub(crate) mod has_singleton;
pub(crate) mod locked_view_elements;
pub mod traits;
pub(crate) mod types;

/// A view across the world that have certain sets of components and singletons
/// locked accordingly
pub struct LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    entities: Arc<RwLock<EntityIdAllocator>>,
    components: C::ComponentSetGuards,
    singletons: S::SingletonContainerGuards,
}

impl<C, S> LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
{
    /// Creates a new locked view
    pub fn new(world: &World) -> Self {
        Self {
            entities: world.entities.clone(),
            components: C::lock_component_sets_from_world(world),
            singletons: S::lock_singleton_containers_from_world(world),
        }
    }

    /// Creates a new entity
    pub fn create_entity(&mut self) -> LockedViewEntity<'_, &mut Self> {
        let id = { self.entities.write().allocate_id() };
        LockedViewEntity::new(id, self)
    }

    /// Gets an entity
    ///
    /// You can still mutate components, just cannot add or remove them
    pub fn get_entity(&self, id: EntityId) -> Option<LockedViewEntity<'_, &Self>> {
        { self.entities.read().index_in_use(id.index) }.then_some(LockedViewEntity::new(id, self))
    }

    /// Gets an entity mutably (allows removing and adding components)
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<LockedViewEntity<'_, &mut Self>> {
        { self.entities.read().index_in_use(id.index) }.then_some(LockedViewEntity::new(id, self))
    }
}

pub(crate) mod private {
    use super::*;

    // GOING TO TAKE A PAUSE ON THIS, NEED TO PUT THE SINGLETON INTO A DANGER CELL!!!
    // TODO: For 2morrow
    // pub trait HasSingleton<T: Singleton, S: LockedViewElements, Idx, QueryIdx>: Sealed {
    //     type Accessor<'a>: Deref<Target = Option<T>> + 'a
    //     where
    //         Self: 'a;

    //     fn get_singleton(&self) -> &Self::Accessor<'_>;
    // }

    // impl<T, C: LockedViewElements, S: LockedViewElements, Idx, QueryIdx> HasSingleton<T, S, Idx, QueryIdx> for LockedView<C, S>
    // where
    //     Idx: 'static,
    //     QueryIdx: 'static,
    //     T: Singleton,
    //     S::Guards: ConsHasOne<Guards<T>, QueryIdx, Idx>,
    //     for<'a> <S::Guards as ConsHasOne<Guards<T>, QueryIdx, Idx>>::Has: Deref<Target = Option<T>> + 'a,
    // {
    //     type Accessor<'a>
    //         = impl Deref<Target = Option<T>>
    //     where
    //         Self: 'a;

    //     fn get_singleton(&self) -> &Self::Accessor<'_> { self.singletons.cons_get_one_ref() }
    // }
}
