use ecs::{
    locked_view::traits::{spawn_ext::LockedViewSpawnExt, *},
    world::{World, traits::spawn_ext::SpawnExt},
};

fn main() {
    let world = World::new();
    world.lock_singleton_entry().or_insert("FOOBAR".to_string()).read();

    world.spawn((300u32, 200i32));

    let _spawned_from_world = world.spawn((512u32, -64i32));

    {
        let mut view = world.lock_view::<(&mut i32, &mut u32, &mut isize), (&mut String,)>();
        view.spawn((32u32, 64i32));

        view.spawn((200u32, 300i32, 23isize));
        view.spawn((12u32, 40i32));

        for (id, (a, b)) in view.query_components::<(&u32, &i32)>() {
            println!("== FILTERING ==");
            println!("{:?}", id);
            println!("{}", *a);
            println!("{}", *b);
            if *a < 100 {
                view.destroy_entity_defered(id);
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
