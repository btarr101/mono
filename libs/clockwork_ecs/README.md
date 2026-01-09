# clockwork_ecs

Entity-component system primitives for the Clockwork workspace. The crate
exposes strongly borrowed views over world data so that systems can lock exactly
the component and singleton rows they require, eliminating runtime borrow
errors.

## Key Concepts

- **World** – Owns entity identifiers plus all component and singleton storage.
- **EntityId** – Stable handles returned by `World::create_entity` or the spawn
  helpers.
- **Components** – Any `Send + Sync + 'static` type automatically implements the
  [`Component`](src/traits/mod.rs) marker trait.
- **Singletons** – Global values stored per type that implement the
  [`Singleton`](src/traits/mod.rs) marker trait.
- **Locked Views** – Scoped borrows that pre-lock component sets and singleton
  containers for the lifetime of the view. Views power spawning, querying, and
  deferred updates.

## Quick Start

```rust
use clockwork_ecs::{
    locked_view::traits::{
        LockedViewGetComponentMutExt,
        LockedViewQueryComponentsOrSingletonsExt,
        LockedViewSpawnExt,
    },
    world::World,
};

#[derive(Default)]
struct Position(f32, f32);
#[derive(Default)]
struct Velocity(f32, f32);

let world = World::new();
let mut view = world.lock_components_view::<(&mut Position, &mut Velocity)>();
let entity = view.spawn((Position::default(), Velocity(1.0, 0.0)));

for (id, (mut position, mut velocity)) in view.query_components::<(&mut Position, &mut Velocity)>() {
    assert_eq!(id, entity);
    position.0 += velocity.0;
}
```

### Working with Singletons

```rust
use clockwork_ecs::{
    locked_view::traits::LockedViewGetSingletonMutExt,
    world::World,
};

#[derive(Default)]
struct FrameCount(u64);

let world = World::new();
let mut view = world.lock_singletons_view::<(&mut FrameCount,)>();
view.singleton_entry::<FrameCount>().or_default().0 += 1;
```

### Deferred Component Updates

Locked views expose methods such as `add_component_defered` and
`remove_component_defered` when you need to stage structural changes.

```rust
use clockwork_ecs::{
    locked_view::traits::LockedViewGetComponentExt,
    world::World,
};

#[derive(Default)]
struct Position(f32, f32);

let world = World::new();
let entity = {
    let view = world.lock_components_view::<(&Position,)>();
    let entity = view.create_entity();
    view.add_component_defered(entity, Position::default());
    world.require_all_and_execute_defered_updates();
    entity
};

let view = world.lock_components_view::<(&Position,)>();
assert!(view.get_component::<Position>(entity).is_some());
```

### Spawning via the World

When you cannot hold onto a locked view, the `SpawnExt` trait bridges world-level
spawning back to locked views:

```rust
use clockwork_ecs::world::{traits::spawn_ext::SpawnExt, World};

#[derive(Default)]
struct Position(f32, f32);

let world = World::new();
let entity = world.spawn((Position::default(),));
assert!(world.entity_exists(entity));
```
