use ecs::{entity::LockedViewEntityComponentMutExt, locked_view::traits::*, world::World};

fn main() {
    let world = World::new();
    world.lock_singleton_entry().or_insert("FOOBAR".to_string()).read();

    let entity = world
        .create_entity()
        .require_components_and_with(20u32)
        .require_components_and_with(13isize)
        .require_components_and_with(200i32);

    {
        let mut view = world.lock_view::<(&mut i32, &mut u32, &mut isize), (&mut String,)>();

        view.create_entity().with(20u32).with(300i32).with(32isize);
        view.create_entity().with(12u32).with(40i32);

        let singleton = view.get_singleton().unwrap();

        // <&String as LockedViewSingletonQueryElement<(&mut i32, &mut u32, &mut isize), (&mut String,), _, _>>::get_singleton_row_element(&view);

        // <(&String, ()) as LockedViewConsSingletonsQuery<
        //     (&mut i32, &mut u32, &mut isize),
        //     (&mut String,),
        //     (Here, ()),
        //     (Here, ()),
        // >>::build_cons_row(&view);

        // TODO: Solve the clone issue

        if let Some(foo) = view.query_singletons::<(&String,)>() {}
        // Likely with OwningHandle creating an owned iterator
        for (id, (comp1, comp2)) in view.query_components::<(&mut u32, &i32)>() {
            // println!("== TOP LEVEL ==");
            // println!("{:?}", id);
            // println!("===============");
            // println!("{}", *a);
            // println!("{}", *b);
            // *a += 1;

            // for (id, (a, b)) in view.query::<(&u32, &i32)>() {
            //     println!("== INNER ==");
            //     println!("{:?}", id);
            //     println!("S: {}", *singleton);
            //     println!("===============");
            //     println!("{}", *a);
            //     println!("{}", *b);

            //     let component = view.get_component_mut::<isize>(id);
            //     println!("{:?}", component.as_deref());
            // }
        }
    }

    entity.require_all_components_and_destroy();

    // for (id, (a, b)) in world.lock_view::<(&i32, &u32), ()>().default_query() {
    //     println!("== TOP LEVEL ENTITY DESTROYED ==");
    //     println!("{:?}", id);
    //     println!("{}", *a);
    //     println!("{}", *b);
    // }
}
