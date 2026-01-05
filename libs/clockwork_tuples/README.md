# tuples

Type-level tuple utilities for building APIs that need compile-time
reasoning over structure, ordering, and membership. The crate exposes a
collection of zero-cost traits, helper types, and macros that let you
navigate tuples as if they were type-level linked lists without paying a
runtime penalty.

## Features

- Structural traits such as `AsConsTuple`, `Has`, `HasOneOf`, `Iter`,
  `Pair`, and `Superset` for expressing tuple invariants in public APIs.
- Index navigation through the `Index`, `Here`, and `There` markers, making
  it possible to pluck elements based solely on their position.
- Conversion helpers for projecting tuples into cons-style forms, allowing
  recursive transformations and flattening via `flat::ToFlat`.
- A tiny `impl_tuple_trait!` macro that fans out implementations across the
  supported tuple arities so you do not have to hand-write boilerplate.
- Zero allocations and zero runtime state: everything is resolved via the
  type system and inlined trait impls.

## Getting started

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tuples = "0.1"
```

## Example

```rust
use clockwork_tuples::traits::{as_cons_tuple::AsConsTuple, has::Has};

fn foo() {
    let tuple = ("alpha", 7u8, true);
    // Convert the tuple into a cons-style representation for structural recursion.
    let cons = tuple.to_cons_tuple();
    assert_eq!(cons.0, "alpha");
    assert_eq!(cons.1.0, 7u8);
    assert!(cons.1.1.0);
    // Navigate to the boolean
    let val: bool = tuple.get();
    assert!(val);
}
```

## Trait surface overview

- `traits::as_cons_tuple::AsConsTuple` – convert tuples into right-nested
  cons lists (owned, shared, or mutable).
- `traits::cons_tuple` – utilities for working directly with cons tuples.
- `traits::has::Has` / `HasOneOf` – index-based and predicate-based element
  accessors.
- `traits::flat::ToFlat` – flatten nested tuples while preserving order.
- `traits::iter` – iterate over homogeneous tuples without heap allocation.
- `traits::pair` – ergonomic helpers for two-element tuples.
- `traits::superset::Superset` – express containment relationships between
  tuple subsets.

## Status & limitations

- The provided macro currently fans out implementations for tuple arities
  up to five elements. Extending to larger tuples just requires updating
  `impl_tuple_trait!` in `src/macros.rs`.
- The crate depends only on the Rust standard library. There are no runtime
  allocations, but `std::marker::PhantomData` is used for index markers.

## Development

```bash
cargo test -p tuples
```

All public modules and traits are documented, and CI will fail on missing
docs thanks to `#![deny(missing_docs)]` in `lib.rs`.
