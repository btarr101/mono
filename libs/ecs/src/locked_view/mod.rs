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

/// Scoped access to a [`World`](crate::world::World) with a fixed set of locked
/// component and singleton guards.
///
/// `LockedView` instances are usually obtained through [`World::lock_view`],
/// [`World::lock_components_view`], or [`World::lock_singletons_view`]. The type
/// parameters `C` and `S` are tuples of component and singleton specifiers (for
/// example `(&Position, &mut Velocity)`) that describe which data is locked.
///
/// # Examples
/// ```no_run
/// use ecs::{
///     locked_view::traits::{LockedViewGetComponentMutExt, LockedViewSpawnExt},
///     world::World,
/// };
///
/// #[derive(Default)]
/// struct Position(f32, f32);
///
/// let world = World::new();
/// let mut view = world.lock_view::<(&mut Position,), ()>();
///
/// let entity = view.spawn((Position::default(),));
/// let mut position = view.get_component_mut::<Position>(entity).unwrap();
/// position.0 = 1.0;
/// ```
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
    /// Creates a locked view by acquiring the component and singleton guards
    /// described by `C` and `S`.
    ///
    /// Most code should prefer the `World::lock_*` helpers which forward to this
    /// constructor.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::locked_view::LockedView;
    /// use ecs::world::World;
    ///
    /// let world = World::new();
    /// let view = LockedView::<(), ()>::new(&world);
    /// assert!(world.entity_exists(view.create_entity()));
    /// ```
    pub fn new(world: &World) -> Self {
        Self {
            entities: world.entities.clone(),
            components: C::lock_component_sets_from_world(world),
            singletons: S::lock_singleton_containers_from_world(world),
            defered_updates: world.defered_updates.clone(),
        }
    }

    /// Allocates a fresh [`EntityId`] without immediately locking component sets.
    ///
    /// The entity becomes visible to other views once deferred updates are
    /// executed.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::locked_view::traits::LockedViewGetComponentExt;
    /// use ecs::world::World;
    ///
    /// #[derive(Default)]
    /// struct Position(f32, f32);
    ///
    /// let world = World::new();
    /// let entity = {
    ///     let view = world.lock_components_view::<(&Position,)>();
    ///     let entity = view.create_entity();
    ///     view.add_component_defered(entity, Position::default());
    ///     entity
    /// };
    /// world.require_all_and_execute_defered_updates();
    /// let view = world.lock_components_view::<(&Position,)>();
    /// assert!(view.get_component::<Position>(entity).is_some());
    /// ```
    pub fn create_entity(&self) -> EntityId { self.entities.write().allocate_id() }

    /// Schedules a component insertion that executes once deferred updates run.
    ///
    /// Use [`World::require_all_and_execute_defered_updates`] to flush deferred
    /// work when you hold the necessary locks.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::locked_view::traits::LockedViewGetComponentExt;
    /// use ecs::world::World;
    ///
    /// #[derive(Default)]
    /// struct Position(f32, f32);
    ///
    /// let world = World::new();
    /// let entity = {
    ///     let view = world.lock_components_view::<(&Position,)>();
    ///     let entity = view.create_entity();
    ///     view.add_component_defered(entity, Position::default());
    ///     entity
    /// };
    /// world.require_all_and_execute_defered_updates();
    ///
    /// let view = world.lock_components_view::<(&Position,)>();
    /// assert!(view.get_component::<Position>(entity).is_some());

    /// ```
    pub fn add_component_defered<T: Component>(&self, id: EntityId, component: T) {
        self.defered_updates.push(
            |(id, component), world| {
                world.require_components_and_add(id, component);
            },
            (id, component),
        );
    }

    /// Schedules a component removal for the given entity.
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::locked_view::traits::LockedViewGetComponentExt;
    /// use ecs::world::World;
    ///
    /// #[derive(Default)]
    /// struct Position(f32, f32);
    ///
    /// let world = World::new();
    /// let entity = {
    ///     let view = world.lock_components_view::<(&Position,)>();
    ///     let entity = view.create_entity();
    ///     view.add_component_defered(entity, Position::default());
    ///     entity
    /// };
    /// world.require_all_and_execute_defered_updates();
    ///
    /// {
    ///     let view = world.lock_components_view::<(&Position,)>();
    ///     view.remove_component_defered::<Position>(entity);
    /// }
    /// world.require_all_and_execute_defered_updates();
    ///
    /// let view = world.lock_components_view::<(&Position,)>();
    /// assert!(view.get_component::<Position>(entity).is_none());
    /// ```
    pub fn remove_component_defered<T: Component>(&self, id: EntityId) {
        self.defered_updates.push(
            |id, world| {
                world.require_components_and_pop::<T>(id);
            },
            id,
        );
    }

    /// Queues entity destruction alongside component cleanup.
    ///
    /// This helper is useful when a system holds component locks that would
    /// otherwise conflict with [`World::require_all_components_and_destroy_entity`].
    ///
    /// # Examples
    /// ```no_run
    /// use ecs::world::World;
    ///
    /// let world = World::new();
    /// let entity = world.create_entity();
    /// {
    ///     let view = world.lock_components_view::<(&i32,)>();
    ///     view.destroy_entity_defered(entity);
    /// }
    /// world.require_all_and_execute_defered_updates();
    /// assert!(!world.entity_exists(entity));
    /// ```
    pub fn destroy_entity_defered(&self, id: EntityId) {
        self.defered_updates.push(
            |id, world| {
                world.require_all_components_and_destroy_entity(id);
            },
            id,
        );
    }
}
