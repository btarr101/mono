#![feature(unsafe_cell_access)]
#![feature(impl_trait_in_assoc_type)]

pub(crate) mod component_set;
pub(crate) mod component_set_guards;
pub(crate) mod danger_cell;
pub mod entity;
pub(crate) mod entity_id_allocator;
pub mod locked_view;
pub(crate) mod sparse_set;
pub mod traits;
pub mod world;

pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        entity::LockedViewEntityComponentMutExt,
        locked_view::{LockedViewComponentsQueryExt, LockedViewGetComponentMutExt},
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
        let mut view = x.lock_view::<(&mut i32, &mut u32, &mut isize)>();
        let entity = view.create_entity().with(20u32).with(300i32);
        view.create_entity().with(12u32).with(40i32);

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

                let s = view.get_component_mut::<isize>(id);

                // let x = view.components::<isize>().get(id);
            }
        }
        drop(view);

        
    }
}
