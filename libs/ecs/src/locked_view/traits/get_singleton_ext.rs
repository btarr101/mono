//! Singleton access extensions for `LockedView`.

use std::ops::{Deref, DerefMut};

use crate::{
    locked_view::{
        LockedView,
        has_singleton::{HasSingleton, HasSingletonMut},
        locked_view_elements::LockedViewElements,
    },
    traits::{
        singleton::Singleton,
        singleton_container_accessor::{
            MutSingletonContainerMutAccessor, SingletonContainerAccessor, SingletonContainerMutAccessor,
        },
    },
};

mod private {
    pub trait Sealed {}
}

/// Provides read-only singleton access through a [`LockedView`](crate::locked_view::LockedView).
///
/// # Examples
/// ```rust
/// use ecs::locked_view::traits::LockedViewGetSingletonExt;
/// use ecs::world::World;
///
/// #[derive(Default)]
/// struct FrameCount(u64);
///
/// let world = World::new();
/// let singleton_view = world.lock_singletons_view::<(&FrameCount,)>();
/// assert!(singleton_view.get_singleton::<FrameCount>().is_none());
/// ```
pub trait LockedViewGetSingletonExt<S: LockedViewElements, Idx, QueryIdx>: private::Sealed {
    /// Returns the singleton if the view locked the requested type.
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>;
}

impl<C: LockedViewElements, S: LockedViewElements> private::Sealed for LockedView<C, S> {}

impl<C, S, Idx, QueryIdx> LockedViewGetSingletonExt<S, Idx, QueryIdx> for LockedView<C, S>
where
    S: LockedViewElements,
    C: LockedViewElements,
    Idx: 'static,
    QueryIdx: 'static,
{
    fn get_singleton<T: Singleton>(&self) -> Option<impl Deref<Target = T>>
    where
        Self: HasSingleton<T, S, Idx, QueryIdx>,
    {
        // SAFETY: `HasSingleton` ensures the accessor references the locked singleton container.
        unsafe { self.get_accessor().get() }
    }
}

/// Mutable handle to an optional singleton stored in a locked view.
///
/// `LockedViewSingletonEntry` mirrors [`std::collections::hash_map::Entry`] APIs
/// and is returned by [`LockedViewGetSingletonMutExt::singleton_entry`].
///
/// # Examples
/// ```rust
/// use ecs::locked_view::traits::LockedViewGetSingletonMutExt;
/// use ecs::world::World;
///
/// #[derive(Default)]
/// struct FrameCount(u64);
///
/// let world = World::new();
/// let mut view = world.lock_singletons_view::<(&mut FrameCount,)>();
/// view
///     .singleton_entry::<FrameCount>()
///     .or_insert_with(FrameCount::default)
///     .0 += 1;
/// ```
pub struct LockedViewSingletonEntry<'a, T: Singleton>(&'a mut Option<T>);

impl<'a, T: Singleton> LockedViewSingletonEntry<'a, T> {
    /// Inserts a singleton value, returning a mutable reference to the stored instance.
    pub fn insert(self, singleton: T) -> impl DerefMut<Target = T> + 'a { self.0.insert(singleton) }

    /// Ensures a singleton exists, returning a mutable reference to the stored instance.
    pub fn or_insert(self, default: T) -> impl DerefMut<Target = T> + 'a { self.0.get_or_insert(default) }

    /// Lazily ensures a singleton exists using the provided constructor.
    pub fn or_insert_with<F>(self, default: F) -> impl DerefMut<Target = T> + 'a
    where
        F: FnOnce() -> T,
    {
        self.0.get_or_insert_with(default)
    }

    /// Ensures a singleton exists using `Default` when absent.
    pub fn or_default(self) -> impl DerefMut<Target = T> + 'a
    where
        T: Default,
    {
        self.0.get_or_insert_default()
    }
}

/// Provides mutable singleton access through a `LockedView`.
///
/// # Examples
/// ```rust
/// use ecs::locked_view::traits::LockedViewGetSingletonMutExt;
/// use ecs::world::World;
///
/// #[derive(Default)]
/// struct FrameCount(u64);
///
/// let world = World::new();
/// let mut view = world.lock_singletons_view::<(&mut FrameCount,)>();
/// view
///     .singleton_entry::<FrameCount>()
///     .or_default()
///     .0 += 1;
/// ```
pub trait LockedViewGetSingletonMutExt<S: LockedViewElements, Idx>: private::Sealed {
    /// Returns a mutable reference to the singleton if locked by the view.
    fn get_singleton_mut<T: Singleton>(&self) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Inserts a singleton and returns a mutable reference to the stored value.
    fn insert_singleton<T: Singleton>(&mut self, singleton: T) -> impl DerefMut<Target = T>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Removes and returns the singleton value if it exists.
    fn pop_singleton<T: Singleton>(&mut self) -> Option<T>
    where
        Self: HasSingletonMut<T, S, Idx>;

    /// Provides a lazy-initialization handle for the singleton.
    fn singleton_entry<T: Singleton>(&mut self) -> LockedViewSingletonEntry<'_, T>
    where
        Self: HasSingletonMut<T, S, Idx>;
}

impl<C, S, Idx> LockedViewGetSingletonMutExt<S, Idx> for LockedView<C, S>
where
    S: LockedViewElements,
    C: LockedViewElements,
    Idx: 'static,
{
    fn get_singleton_mut<T: Singleton>(&self) -> Option<impl DerefMut<Target = T>>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        // SAFETY: `HasSingletonMut` ensures exclusive access to the singleton container.
        unsafe { self.get_accessor().get_mut() }
    }

    fn insert_singleton<T: Singleton>(&mut self, singleton: T) -> impl DerefMut<Target = T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        // SAFETY: The mutable accessor owns the lock for insertion.
        unsafe { self.get_mut_accessor().insert(singleton) }
    }

    fn pop_singleton<T: Singleton>(&mut self) -> Option<T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        // SAFETY: The mutable accessor holds exclusive access to remove the singleton.
        unsafe { self.get_mut_accessor().pop() }
    }

    fn singleton_entry<T: Singleton>(&mut self) -> LockedViewSingletonEntry<'_, T>
    where
        Self: HasSingletonMut<T, S, Idx>,
    {
        // SAFETY: Exclusive access allows exposing the entry for deferred initialization.
        LockedViewSingletonEntry(unsafe { self.get_mut_accessor().get_entry() })
    }
}
