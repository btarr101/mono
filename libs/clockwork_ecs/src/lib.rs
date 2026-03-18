//! Entity–component system core primitives.
//!
//! This crate provides low-level building blocks for constructing an
//! entity–component system with strict borrowing, aliasing, and access
//! guarantees enforced at compile time.
//!
//! The public API is intended for library consumers building ECS-based
//! runtimes or engines. Internal modules may rely on unsafe code to
//! uphold documented invariants.

#![feature(impl_trait_in_assoc_type)]

pub mod entity_id;
pub mod locked_view;
pub mod traits;
pub(crate) mod util;
pub mod world;
