use crate::traits::prepend::CanPrepend;

pub trait Flat {
    type Flattened: CanPrepend;

    fn flatten(self) -> Self::Flattened;
}

impl Flat for () {
    type Flattened = ();

    fn flatten(self) -> Self::Flattened {}
}

impl<Head, Tail> Flat for (Head, Tail)
where
    Tail: Flat,
{
    type Flattened = <Tail::Flattened as CanPrepend>::Prepended<Head>;

    fn flatten(self) -> Self::Flattened { self.1.flatten().prepend(self.0) }
}
