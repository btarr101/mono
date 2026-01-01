use crate::{
    indexes::{Here, Index, There},
    traits::{as_cons_tuple::AsConsTuple, flat::Flat},
};

/// Trait for if a tuple contains an element.
pub trait Has<T, Idx> {
    type Plucked;

    /// Deconstructs the original tuple and gets the element
    fn get(self) -> T;
    /// Deconstructs this tuple into a plucked element and the rest of the tuple
    fn plucked(self) -> (T, Self::Plucked);
    /// Gets a reference to the element in this tuple
    fn get_ref(&self) -> &T;
    /// Gets a mutable reference to the element in this tuple
    fn get_mut(&mut self) -> &mut T;
}

impl<Tuple, T, Idx> Has<T, Idx> for Tuple
where
    Tuple: AsConsTuple,
    Tuple::As: ConsHas<T, Idx>,
    <Tuple::As as ConsHas<T, Idx>>::Plucked: Flat,
    for<'a> Tuple::AsRefs<'a>: ConsHas<&'a T, Idx>,
    for<'a> Tuple::AsMuts<'a>: ConsHas<&'a mut T, Idx>,
{
    type Plucked = <<Tuple::As as ConsHas<T, Idx>>::Plucked as Flat>::Flattened;

    fn get(self) -> T { self.to_cons_tuple().cons_get() }
    fn plucked(self) -> (T, Self::Plucked) {
        let (target, plucked) = self.to_cons_tuple().cons_pluck();
        (target, plucked.flatten())
    }
    fn get_ref(&self) -> &T { self.to_cons_ref_tuple().cons_get() }
    fn get_mut(&mut self) -> &mut T { self.to_cons_mut_tuple().cons_get() }
}

/// Trait for if a cons style tuple contains an element.
pub trait ConsHas<T, Idx> {
    type Plucked;

    /// Deconstructs the cons tuple and gets the element
    fn cons_get(self) -> T;
    /// Deconstructs the cons tuple into a plucked element and the rest of the tuple
    fn cons_pluck(self) -> (T, Self::Plucked);
    /// Gets a reference to the element in this cons tuple
    fn cons_get_ref(&self) -> &T;
    /// Gets a mutable reference to the element in this cons tuple
    fn cons_get_mut(&mut self) -> &mut T;
}

impl<Target, Tail> ConsHas<Target, Here> for (Target, Tail) {
    type Plucked = Tail;

    fn cons_get(self) -> Target { self.0 }
    fn cons_pluck(self) -> (Target, Self::Plucked) { self }
    fn cons_get_ref(&self) -> &Target { &self.0 }
    fn cons_get_mut(&mut self) -> &mut Target { &mut self.0 }
}

impl<Head, Tail, Target, Idx: Index> ConsHas<Target, There<Idx>> for (Head, Tail)
where
    Tail: ConsHas<Target, Idx>,
{
    type Plucked = (Head, Tail::Plucked);

    fn cons_get(self) -> Target { self.1.cons_get() }
    fn cons_pluck(self) -> (Target, Self::Plucked) {
        let (target, plucked) = self.1.cons_pluck();
        (target, (self.0, plucked))
    }
    fn cons_get_ref(&self) -> &Target { self.1.cons_get_ref() }
    fn cons_get_mut(&mut self) -> &mut Target { self.1.cons_get_mut() }
}
