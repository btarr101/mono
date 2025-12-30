use std::cell::{Ref, RefMut};

use crate::{danger_cell::DangerCell, entity::EntityId, sparse_set::SparseSet, traits::component::Component};

/// Internal data structure for storring components
#[derive(Default)]
pub struct ComponentSet<T: Component>(SparseSet<(usize, DangerCell<T>)>);

impl<T: Component> ComponentSet<T> {
    /// Creates a new component set
    pub fn new() -> Self { Self(SparseSet::new()) }

    /// Gets a component from this set given the entity id
    ///
    /// # Safety
    /// Only call if no thread thinks it has exclusive access
    pub unsafe fn get_shared(&self, id: EntityId) -> Option<&T> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation).then_some(unsafe { component.get_shared() })
    }

    /// Gets a component from this set given the entity id
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_exclusive(&self, id: EntityId) -> Option<Ref<'_, T>> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation).then_some(unsafe { component.get() }).flatten()
    }

    /// Same as `get_exclusive` but mutable
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_mut_exclusive(&self, id: EntityId) -> Option<RefMut<'_, T>> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation)
            .then_some(unsafe { component.get_mut() })
            .flatten()
    }

    /// Adds a component to this component set
    pub fn add(&mut self, id: EntityId, component: T) { self.0.add(id.index, (id.generation, component.into())); }

    /// Removes a component from this component set, and if a component
    /// was removed this way returns it
    pub fn pop(&mut self, id: EntityId) -> Option<T> {
        let (generation, _) = self.0.get(id.index)?;
        if *generation != id.generation {
            return None;
        }

        self.0.pop(id.index).map(|(_, cell)| cell.into_inner())
    }

    /// Iterates through every component in this set
    ///
    /// # Safety
    /// Only call if no thread has exclusive access
    pub unsafe fn iter_shared(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.0.iter().map(|(index, (generation, component))| {
            (
                EntityId {
                    index,
                    generation: *generation,
                },
                unsafe { component.get_shared() },
            )
        })
    }

    /// Iterates through every component in this set
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn iter_exclusive(&self) -> impl Iterator<Item = (EntityId, Ref<'_, T>)> {
        self.0.iter().filter_map(|(index, (generation, component))| {
            (
                EntityId {
                    index,
                    generation: *generation,
                },
                unsafe { component.get() }?,
            )
                .into()
        })
    }

    /// Iterates through every component in this set mutably
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn iter_mut_exclusive(&self) -> impl Iterator<Item = (EntityId, RefMut<'_, T>)> {
        self.0.iter().filter_map(|(index, (generation, component))| {
            (
                EntityId {
                    index,
                    generation: *generation,
                },
                unsafe { component.get_mut() }?,
            )
                .into()
        })
    }
}
