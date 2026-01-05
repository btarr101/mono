//! Traits describing structural properties of tuple types.
//!
//! This module defines the public trait surface for reasoning about
//! tuple composition, containment, iteration order, and transformation
//! at the type level.
//!
//! # Invariants
//! - All traits are zero-sized and impose no runtime cost.
//! - Trait implementations must reflect tuple structure exactly.
//! - No trait in this module performs allocation or mutation.

/// Conversion to cons-style tuples.
pub mod as_cons_tuple;
/// Tuple element prepending.
pub mod can_prepend;
/// Cons-style tuple representation.
pub mod cons_tuple;
/// Flattening nested tuples.
pub mod flat;
/// Indexed tuple access.
pub mod has;
/// Selecting a single matching element.
pub mod has_one_of;
/// Iteration over homogeneous tuples.
pub mod iter;
/// Access to two-element tuples.
pub mod pair;
/// Subset extraction from tuples.
pub mod superset;
