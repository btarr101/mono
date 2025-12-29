use crate::{
    indexes::{Here, Index, There},
    traits::as_cons_tuple::AsConsTuple,
};

/// Trait for if a tuple contains an element.
pub trait Has<T, Idx> {
    /// Deconstructs the original tuple and gets the element
    fn get(self) -> T;
    /// Gets a reference to the element in this tuple
    fn get_ref(&self) -> &T;
    /// Gets a mutable reference to the element in this tuple
    fn get_mut(&mut self) -> &mut T;
}

impl<Tuple, T, Idx> Has<T, Idx> for Tuple
where
    Tuple: AsConsTuple,
    Tuple::As: ConsHas<T, Idx>,
    for<'a> Tuple::AsRefs<'a>: ConsHas<&'a T, Idx>,
    for<'a> Tuple::AsMuts<'a>: ConsHas<&'a mut T, Idx>,
{
    fn get(self) -> T { self.to_cons_tuple().cons_get() }
    fn get_ref(&self) -> &T { self.to_cons_ref_tuple().cons_get() }
    fn get_mut(&mut self) -> &mut T { self.to_cons_mut_tuple().cons_get() }
}

/// Trait for if a cons style tuple contains an element.
pub trait ConsHas<T, Idx> {
    /// Deconstructs the cons tuple and gets the element
    fn cons_get(self) -> T;
    /// Gets a reference to the element in this cons tuple
    fn cons_get_ref(&self) -> &T;
    /// Gets a mutable reference to the element in this cons tuple
    fn cons_get_mut(&mut self) -> &mut T;
}

impl<Target, Tail> ConsHas<Target, Here> for (Target, Tail) {
    fn cons_get(self) -> Target { self.0 }
    fn cons_get_ref(&self) -> &Target { &self.0 }
    fn cons_get_mut(&mut self) -> &mut Target { &mut self.0 }
}

impl<Head, Tail, Target, Idx: Index> ConsHas<Target, There<Idx>> for (Head, Tail)
where
    Tail: ConsHas<Target, Idx>,
{
    fn cons_get(self) -> Target { self.1.cons_get() }
    fn cons_get_ref(&self) -> &Target { self.1.cons_get_ref() }
    fn cons_get_mut(&mut self) -> &mut Target { self.1.cons_get_mut() }
}
