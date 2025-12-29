#![feature(unsafe_cell_access)]
#![feature(impl_trait_in_assoc_type)]

pub mod component_set;
pub mod component_set_guards;
pub mod danger_cell;
pub mod entity;
pub mod locked_view;
pub mod sparse_set;
pub mod traits;
pub mod world;

pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        entity::EntityId,
        locked_view::{LockedViewComponentsMutExt, LockedViewComponentsQueryExt},
        traits::component_set_accessor::MutComponentSetMutAccessor,
        world::World,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        let x = World::new();
        let mut view = x.lock_view::<(&mut i32, &mut u32)>();
        view.mut_components_mut().add(EntityId::new(10, 0), 20u32);
        view.mut_components_mut().add(EntityId::new(10, 0), 300i32);

        view.mut_components_mut().add(EntityId::new(12, 0), 200u32);
        view.mut_components_mut().add(EntityId::new(12, 0), 300i32);

        // for y in x {}
        for (id, (mut a, b)) in view.query::<(&mut u32, &i32)>() {
            println!("{:?}", id);
            println!("{}", *a);
            println!("{}", *b);
            *a += 1;

            for (id, (a, b)) in view.query::<(&u32, &i32)>() {
                println!("{:?}", id);
                println!("{}", *a);
                println!("{}", *b);
            }
        }

        // let y = <locked_view::LockedView<(&mut i32, &mut u32)> as locked_view::LockedViewComponentsQueryExt<
        //     (&mut i32, &mut u32),
        //     (There<Here>, ()),
        //     _,
        // >>::query::<(&u32, ())>(&view);

        // let z = view.components::<u32>();

        // // z.get_mut(EntityId::new(10, 4));
        // for (_, entity) in z.iter() {
        //     dbg!(*entity);
        // }
        // drop(z);
        // let mut g = view.mut_components_mut::<u32>();
        // g.add(EntityId::new(10, 4), 32);
        // g.get_mut(EntityId::new(10, 4));

        // for (id, entity) in g.iter() {
        //     println!("{:?}", id);
        //     println!("{}", *entity);
        // }

        // print!("FOO");
        // let query = <LockedView<(&mut i32, &mut u32)> as LockedViewComponentsQueryExt<
        //     (&mut i32, &mut u32),
        //     (There<Here>, (Here, ())),
        //     (There<Here>, (There<Here>, ())),
        // >>::query::<(&u32, (&i32, ()))>(&view)
        // .map(|(id, (a, (b, _)))| (id, *a, *b))
        // .collect::<Vec<_>>();

        // dbg!(&query);

        // for x in view.query::<(&u32, ())>() {}
    }
}
