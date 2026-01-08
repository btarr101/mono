use ecs::{
    entity::LockedViewEntityComponentMutExt,
    locked_view::traits::{spawn_ext::LockedViewSpawnExt, *},
    world::World,
};

fn main() {
    let world = World::new();
    world.lock_singleton_entry().or_insert("FOOBAR".to_string()).read();

    world
        .create_entity()
        .require_components_and_with(300u32)
        .require_components_and_with(13isize)
        .require_components_and_with(200i32);

    {
        let mut view = world.lock_view::<(&mut i32, &mut u32, &mut isize), (&mut String,)>();
        let entity = view.spawn((32u32, 64i32));

        view.create_entity().with(200u32).with(300i32).with(32isize);
        view.create_entity().with(12u32).with(40i32);

        for (id, (a, b)) in view.query_components::<(&u32, &i32)>() {
            println!("== FILTERING ==");
            println!("{:?}", id);
            println!("{}", *a);
            println!("{}", *b);
            if *a < 100
                && let Some(entity) = view.get_entity(id)
            {
                entity.destroy_defered();
            }
        }
    }

    world.require_all_and_execute_defered_updates();

    for (id, (a, b)) in world
        .lock_components_view::<(&i32, &u32)>()
        .query_components::<(&i32, &u32)>()
    {
        println!("== TOP LEVEL ENTITY DESTROYED ==");
        println!("{:?}", id);
        println!("{}", *a);
        println!("{}", *b);
    }
}
