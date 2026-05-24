#![deny(missing_docs)]

//! Tuple-oriented type-level utilities.
//!
//! This crate provides traits and helpers for reasoning about and
//! manipulating tuple types at the type level. It is intended for
//! library consumers that need compile-time guarantees over tuple
//! structure, ordering, and containment.
//!
//! # Guarantees
//!
//! - All public traits define structural properties of tuple types.
//! - No runtime allocation is performed by this crate.
//! - APIs are pure with respect to observable runtime state.
//!
//! # Examples
//! ```
//! use clockwork_tuples::traits::{
//!     as_cons_tuple::AsConsTuple,
//!     as_ref_tuple::AsRefTuple,
//!     has::Has,
//! };
//!
//! fn foo() {
//!     let tuple = ("alpha", 7u8, true);
//!
//!     // Convert the tuple into a cons-style representation for structural recursion.
//!     let cons = tuple.to_cons_tuple();
//!     assert_eq!(cons.0, "alpha");
//!     assert_eq!(cons.1.0, 7u8);
//!     assert!(cons.1.1.0);
//!
//!     // Navigate to the boolean
//!     let val: bool = tuple.get();
//!     assert!(val);
//! }
//!
//! fn borrow_refs() {
//!     let mut borrowable = ("bravo".to_string(), 3u8, false);
//!
//!     // Borrow shared refs without moving ownership.
//!     {
//!         let (label, count, flag) = borrowable.as_refs();
//!         assert_eq!(label.as_str(), "bravo");
//!         assert_eq!(*count, 3u8);
//!         assert!(!*flag);
//!     }
//!
//!     // Borrow mutable refs to update individual elements in place.
//!     {
//!         let (label_mut, count_mut, flag_mut) = borrowable.as_muts();
//!         label_mut.push_str("-borrowed");
//!         *count_mut += 1;
//!         *flag_mut = true;
//!     }
//!
//!     assert_eq!(borrowable.0, "bravo-borrowed");
//!     assert_eq!(borrowable.1, 4u8);
//!     assert!(borrowable.2);
//! }
//! ```

pub mod index;
pub(crate) mod macros;
pub mod traits;
