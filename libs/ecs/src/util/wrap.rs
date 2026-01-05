//! Lightweight newtype that preserves `Deref` semantics.

use derived_deref::{Deref, DerefMut};

/// Newtype that forwards `Deref`/`DerefMut` to the wrapped value.
#[derive(Deref, DerefMut)]
pub struct Wrap<T>(pub T);
