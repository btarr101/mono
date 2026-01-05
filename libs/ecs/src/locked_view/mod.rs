//! Locked views over world state.
//!
//! A locked view provides scoped access to a `World` with a fixed set of
//! component and singleton locks held for the lifetime of the view.

use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

use crate::{
    entity::{EntityId, LockedViewEntity},
    locked_view::locked_view_elements::LockedViewElements,
    util::defered_queue::DeferedQueue,
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
    pub(crate) defered_updates: Arc<Mutex<DeferedQueue<World>>>,
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
            defered_updates: world.defered_updates.clone(),
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
