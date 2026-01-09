//! Component access extensions for `LockedView`.

use std::ops::{Deref, DerefMut};

use crate::{
    entity_id::EntityId,
    locked_view::{
        LockedView,
        has_components::{HasComponents, HasComponentsMut},
        locked_view_elements::LockedViewElements,
    },
    traits::{
        component::Component,
        component_set_accessor::{ComponentSetAccessor, ComponentSetMutAccessor, MutComponentSetMutAccessor},
    },
};

mod private {
    pub trait Sealed {}
}

/// Provides read-only component access through a [`LockedView`](crate::locked_view::LockedView).
///
/// # Examples
/// ```rust
/// use clockwork_ecs::locked_view::traits::LockedViewGetComponentExt;
/// use clockwork_ecs::world::World;
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
/// let position = view.get_component::<Position>(entity).unwrap();
/// assert_eq!(position.0, 0.0);
/// ```
pub trait LockedViewGetComponentExt<C: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Returns the component associated with `id` if the current view locked it.
    fn get_component<T: Component>(&self, id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: HasComponents<T, C, Idx, QueryIdx>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<C, S, Idx, QueryIdx> LockedViewGetComponentExt<C, Idx, QueryIdx> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idx: 'static,
    QueryIdx: 'static,
{
    fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<impl Deref<Target = T>>
    where
        Self: HasComponents<T, C, Idx, QueryIdx>,
    {
        // SAFETY: `HasComponents` ensures the accessor references the locked
        // component set for the queried entity.
        unsafe { self.get_accessor().get(entity_id) }
    }
}

/// Provides mutable component access through a `LockedView`.
///
/// # Examples
/// ```rust
/// use clockwork_ecs::locked_view::traits::{LockedViewGetComponentMutExt, LockedViewSpawnExt};
/// use clockwork_ecs::world::World;
///
/// #[derive(Default)]
/// struct Position(f32, f32);
///
/// let world = World::new();
/// let mut view = world.lock_view::<(&mut Position,), ()>();
/// let entity = view.spawn((Position::default(),));
///
/// let mut position = view.get_component_mut::<Position>(entity).unwrap();
/// position.0 = 42.0;
/// ```
pub trait LockedViewGetComponentMutExt<C: LockedViewElements, Idx>: private::Sealed {
    /// Returns a mutable reference to the component associated with `id` if present.
    fn get_component_mut<T: Component>(&self, id: EntityId) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>;

    /// Attempts to insert a component for `id` and returns a mutable reference on success.
    ///
    /// Callers must handle the `None` case to detect when the component was not inserted
    /// (for example when another system already inserted the component).
    #[must_use]
    fn add_component<T: Component>(&mut self, id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>;

    /// Removes the component associated with `id`, returning it if present.
    fn pop_component<T: Component>(&mut self, id: EntityId) -> Option<T>
    where
        Self: HasComponentsMut<T, C, Idx>;
}

impl<C, S, Idx> LockedViewGetComponentMutExt<C, Idx> for LockedView<C, S>
where
    C: LockedViewElements,
    S: LockedViewElements,
    Idx: 'static,
{
    fn get_component_mut<T: Component>(&self, entity_id: EntityId) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        // SAFETY: `HasComponentsMut` ensures mutable access is unique for the
        // component set referenced by this view.
        unsafe { self.get_accessor().get_mut(entity_id) }
    }

    fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        // SAFETY: The mutable accessor owns the component set lock, so inserting
        // a component maintains aliasing guarantees.
        unsafe { self.get_mut_accessor().try_add(entity_id, component) }
    }

    fn pop_component<T: Component>(&mut self, entity_id: EntityId) -> Option<T>
    where
        Self: HasComponentsMut<T, C, Idx>,
    {
        self.get_mut_accessor().soft_pop(entity_id)
    }
}
