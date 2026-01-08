//! Locked views over world state.
//!
//! A locked view provides scoped access to a `World` with a fixed set of
//! component and singleton locks held for the lifetime of the view.

use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    entity_id::EntityId,
    locked_view::locked_view_elements::LockedViewElements,
    traits::component::Component,
    util::defered_queue::RotatingLockedDeferedQueue,
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
    pub(crate) defered_updates: Arc<RotatingLockedDeferedQueue<World>>,
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
    pub fn create_entity(&self) -> EntityId { self.entities.write().allocate_id() }

    /// Adds a component to an entity in defered fasion
    pub fn add_component_defered<T: Component>(&self, id: EntityId, component: T) {
        self.defered_updates.push(
            |(id, component), world| {
                world.require_components_and_add(id, component);
            },
            (id, component),
        );
    }

    /// Removes a component from an entity in defered fasion
    pub fn remove_component_defered<T: Component>(&self, id: EntityId) {
        self.defered_updates.push(
            |id, world| {
                world.require_components_and_pop::<T>(id);
            },
            id,
        );
    }

    /// Destroys an entity in defered fasion
    pub fn destroy_entity_defered(&self, id: EntityId) {
        self.defered_updates.push(
            |id, world| {
                world.require_all_components_and_destroy_entity(id);
            },
            id,
        );
    }
}
