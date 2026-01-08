use clockwork_tuples::traits::as_cons_tuple::AsConsTuple;

use crate::{
    entity::Entity,
    traits::{
        component::Component,
        cons_guards::{ConsAsComponentSetGuards, ConsGuards},
    },
    world::World,
};

pub trait ConsSpawnTuple {
    type ComponentSetGuards;

    fn lock_component_sets_from_world(world: &World) -> Self::ComponentSetGuards;
    fn spawn(world: &World) -> Entity<'_>;
}

impl ConsSpawnTuple for () {
    type ComponentSetGuards = ();

    fn lock_component_sets_from_world(_: &World) -> Self::ComponentSetGuards {}
    fn spawn(world: &World) -> Entity<'_> { world.create_entity() }
}

// impl<Head, Tail> ConsSpawnTuple for (Head, Tail)
// where
//     Head: Component,
//     Tail: ConsSpawnTuple,
// {
//     fn spawn(world: &World) -> Entity<'_> {
//         let mut entities = world.entities.write();
//         let id = entities.allocate_id();

// 		world.lock
//         todo!()
//     }
// }
