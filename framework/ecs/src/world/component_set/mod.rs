use std::{
    any::Any,
    cell::{Ref, RefMut},
};

use crate::{
    entity::EntityId,
    traits::component::Component,
    util::{danger_cell::DangerCell, sparse_set::SparseSet},
};

pub(crate) mod component_set_guards;

/// Internal data structure for storring components
#[derive(Default)]
pub struct ComponentSet<T: Component>(SparseSet<(usize, Option<DangerCell<T>>)>);

pub trait AnyComponentSet: Any + Send + Sync {
    /// Removes a component from this component set
    /// given it's index
    fn remove(&mut self, index: usize);
}

impl<T: Component> ComponentSet<T> {
    /// Creates a new component set
    pub fn new() -> Self { Self(SparseSet::new()) }

    /// Gets a component from this set given the entity id
    ///
    /// # Safety
    /// Only call if no thread thinks it has exclusive access
    pub unsafe fn get_shared(&self, id: EntityId) -> Option<&T> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation).then_some(unsafe { component.as_ref()?.get_shared() })
    }

    /// Gets a component from this set given the entity id
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_exclusive(&self, id: EntityId) -> Option<Ref<'_, T>> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation)
            .then_some(unsafe { component.as_ref()?.get() })
            .flatten()
    }

    /// Same as `get_exclusive` but mutable
    ///
    /// # Safety
    /// Only call if this thread has exclusive access
    pub unsafe fn get_mut_exclusive(&self, id: EntityId) -> Option<RefMut<'_, T>> {
        let (generation, component) = self.0.get(id.index)?;
        (*generation == id.generation)
            .then_some(unsafe { component.as_ref()?.get_mut() })
            .flatten()
    }

    /// Adds a component to this component set and returns an immediate reference
    ///
    /// Disregards if the current generation of the entity id matches up or not
    pub fn add(&mut self, id: EntityId, component: T) -> &mut T {
        let cell = self
            .0
            .add(id.index, (id.generation, Some(component.into())))
            .1
            .as_mut()
            .expect("component");

        unsafe { cell.get_mut_exclusive() }
    }

    /// Attempts to add a component to this component set
    ///
    /// If a component was added this way returns an immediate reference to it
    pub fn try_add(&mut self, id: EntityId, component: T) -> Option<&mut T> {
        let current_generation = self.0.get(id.index).map(|(generation, _)| *generation)?;

        (current_generation == id.generation).then(|| self.add(id, component))
    }

    /// Removes a component from this component set by index. If there was a genration,
    /// returns it, and then if there was a component, returns it.
    ///
    /// (essentially this should be called when the entity is removed entirely)
    pub fn pop(&mut self, index: usize) -> Option<(usize, Option<T>)> {
        self.0
            .pop(index)
            .map(|(generation, cell)| (generation, cell.map(|cell| cell.into_inner())))
    }

    /// Attempts to removes a component from this component set.
    /// If a component is removed this way returns it.
    ///
    /// We have this in addition to pop because this respects the generation of the
    /// the entity id and the generation that is stored. This is important because it disallows
    ///
    /// Removing a component from an entity id with index 4 gen 3
    /// Adding a component to an entity id with index 4 gen 5
    ///
    /// The previous generation must be cleared out before another component can be added
    pub fn soft_pop(&mut self, id: EntityId) -> Option<T> {
        let (generation, cell) = self.0.get_mut(id.index)?;
        (*generation == id.generation)
            .then(|| cell.take().map(|cell| cell.into_inner()))
            .flatten()
    }

    /// Iterates through every component in this set
    ///
    /// # Safety
    /// Only call if no thread has exclusive access
    pub unsafe fn iter_shared(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.0.iter().filter_map(|(index, (generation, component))| {
            (
                EntityId {
                    index,
                    generation: *generation,
                },
                unsafe { component.as_ref()?.get_shared() },
            )
                .into()
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
                unsafe { component.as_ref()?.get() }?,
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
                unsafe { component.as_ref()?.get_mut() }?,
            )
                .into()
        })
    }
}

impl<T: Component> AnyComponentSet for ComponentSet<T> {
    fn remove(&mut self, index: usize) { self.pop(index); }
}
