#![deny(missing_docs)]

//! Tuple-oriented type-level utilities.
//!
//! This crate provides traits and helpers for reasoning about and
//! manipulating tuple types at the type level. It is intended for
//! library consumers that need compile-time guarantees over tuple
//! structure, ordering, and containment.
//!
//! # Guarantees
//! - All public traits define structural properties of tuple types.
//! - No runtime allocation is performed by this crate.
//! - APIs are pure with respect to observable runtime state.

pub mod index;
pub(crate) mod macros;
pub mod traits;
