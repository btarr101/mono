use ecs::{
    entity::LockedViewEntityComponentMutExt,
    locked_view::{LockedViewComponentsQueryExt, LockedViewGetComponentMutExt},
    world::World,
};

fn main() {
    let world = World::new();
    let entity = world
        .create_entity()
        .lock_components_and_with(20u32)
        .lock_components_and_with(13isize)
        .lock_components_and_with(200i32);

    {
        let mut view = world.lock_view::<(&mut i32, &mut u32, &mut isize)>();
        view.create_entity().with(20u32).with(300i32);
        view.create_entity().with(12u32).with(40i32);

        for (id, (mut a, b)) in view.query::<(&mut u32, &i32)>() {
            println!("== TOP LEVEL ==");
            println!("{:?}", id);
            println!("===============");
            println!("{}", *a);
            println!("{}", *b);
            *a += 1;

            for (id, (a, b)) in view.query::<(&u32, &i32)>() {
                println!("== INNER ==");
                println!("{:?}", id);
                println!("===============");
                println!("{}", *a);
                println!("{}", *b);

                let component = view.get_component_mut::<isize>(id);
                println!("{:?}", component.as_deref());
            }
        }
    }

    entity.lock_all_components_and_destroy();

    for (id, (a, b)) in world.lock_view::<(&i32, &u32)>().default_query() {
        println!("== TOP LEVEL ENTITY DESTROYED ==");
        println!("{:?}", id);
        println!("{}", *a);
        println!("{}", *b);
    }
}
