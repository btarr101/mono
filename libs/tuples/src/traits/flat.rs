use crate::traits::can_prepend::CanPrepend;

pub trait ToFlat {
    type Flattened: CanPrepend;

    fn flatten(self) -> Self::Flattened;
}

impl ToFlat for () {
    type Flattened = ();

    fn flatten(self) -> Self::Flattened {}
}

impl<Head, Tail> ToFlat for (Head, Tail)
where
    Tail: ToFlat,
{
    type Flattened = <Tail::Flattened as CanPrepend>::Prepended<Head>;

    fn flatten(self) -> Self::Flattened { self.1.flatten().prepend(self.0) }
}
