# sparsr

> A sparse set data structure.

A sparse set stores key/value pairs where the key is a small index and the value
can be anything. It assumes keys are much smaller than values, so storing values
under sparse or widely spread indices costs little.

Lookup, insertion, and removal are all O(1), and iteration walks the values
densely rather than scanning the whole key range.

## Installation

```bash
cargo add sparsr
```

## Usage

```rust
use sparsr::prelude::SparseSet;

let mut set = SparseSet::<&str>::new();
set.insert(3, "three");
set.insert(1, "one");

assert_eq!(set.get(3), Some(&"three"));
assert_eq!(set.pop(1), Some("one"));
assert_eq!(set.len(), 1);
```

Iteration yields entries in ascending index order. For in-place manipulation,
`entry` returns a `SparseSetEntry`, which is either occupied or vacant, mirroring
the standard library's entry API.
