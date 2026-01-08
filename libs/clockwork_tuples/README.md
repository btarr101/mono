# clockwork-tuples

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
- Borrow-friendly adapters such as `traits::as_ref_tuple::AsRefTuple`, which
  expose `as_refs`/`as_muts` so callers can share or mutate tuple contents
  without moving ownership.
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
use clockwork_tuples::traits::{
    as_cons_tuple::AsConsTuple,
    as_ref_tuple::AsRefTuple,
    has::Has,
};

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

fn borrow_refs() {
    let mut borrowable = ("bravo".to_string(), 3u8, false);

    // Borrow shared refs without moving ownership.
    {
        let (label, count, flag) = borrowable.as_refs();
        assert_eq!(label.as_str(), "bravo");
        assert_eq!(*count, 3u8);
        assert!(!*flag);
    }

    // Borrow mutable refs to update individual elements in place.
    {
        let (label_mut, count_mut, flag_mut) = borrowable.as_muts();
        label_mut.push_str("-borrowed");
        *count_mut += 1;
        *flag_mut = true;
    }

    assert_eq!(borrowable.0, "bravo-borrowed");
    assert_eq!(borrowable.1, 4u8);
    assert!(borrowable.2);
}
```

## Trait surface overview

- `traits::as_cons_tuple::AsConsTuple` – convert tuples into right-nested
  cons lists (owned, shared, or mutable).
- `traits::as_ref_tuple::AsRefTuple` – derive shared or mutable references to
  tuple members without moving the original tuple.
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
