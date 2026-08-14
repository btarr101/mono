# griddy

> We need dem grids

Griddy is a simple store for a 2D grid of cells. What's unique about it is it doesn't care about having fixed dimensions.
Logically, the grid is infinitely sized. But physically, the grid starts out with the center at (0, 0) and has dimensions of (0, 0).

- If you need to immediately add a cell at (-5678, 42), go for it!
- If you need to access cell (2045, 29) right way, go for it!
- If you need to **mutably** access cell (3456, 666), go for it!

Because of this, cell's must implement `Default` (for filling in the empty cells when the grid needs to expand) and `Clone` (because duh).

## Why

I'm making a game with grids. I need 'em.

## Installation

```bash
cargo add griddy
```

## Usage

```rust
use griddy::Grid;

let mut grid = Grid::<isize>::new();
let previous = grid.upsert((-5678, 42), 42);

assert_eq!(previous, 0);
```
