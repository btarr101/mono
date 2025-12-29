use crate::traits::{as_cons_tuple::AsConsTuple, flat::Flat, prepend::Prepend};

/// Trait to convert a tuple to a tuple of references
pub trait AsRefs {
    type Refs<'a>
    where
        Self: 'a;
    type Muts<'a>
    where
        Self: 'a;

    fn refs(&self) -> Self::Refs<'_>;
    fn muts(&mut self) -> Self::Muts<'_>;
}

impl<Tuple> AsRefs for Tuple
where
    Self: AsConsTuple,
{
    type Refs<'a>
        = <<Self as AsConsTuple>::AsRefs<'a> as Flat>::Flattened
    where
        Self: 'a;
    type Muts<'a>
        = <<Self as AsConsTuple>::AsMuts<'a> as Flat>::Flattened
    where
        Self: 'a;

    fn refs(&self) -> Self::Refs<'_> { self.to_cons_ref_tuple().flatten() }
    fn muts(&mut self) -> Self::Muts<'_> { self.to_cons_mut_tuple().flatten() }
}

/// Trait for converting a cons style tuple to a cons style tuple of refs
pub trait ConsAsRefs {
    type Refs<'a>: Prepend
    where
        Self: 'a;
    type Muts<'a>: Prepend
    where
        Self: 'a;

    /// Gets this same cons tuple, but as a tuple of refs
    fn refs(&self) -> Self::Refs<'_>;

    /// Gets this same cons tuple, but as a tuple of muts
    fn muts(&mut self) -> Self::Muts<'_>;
}

impl ConsAsRefs for () {
    type Refs<'a> = ();
    type Muts<'a> = ();
    fn refs(&self) -> Self::Refs<'_> {}
    fn muts(&mut self) -> Self::Muts<'_> {}
}

impl<Head, Tail> ConsAsRefs for (Head, Tail)
where
    Tail: ConsAsRefs,
{
    type Refs<'a>
        = (&'a Head, Tail::Refs<'a>)
    where
        Self: 'a;
    type Muts<'a>
        = (&'a mut Head, Tail::Muts<'a>)
    where
        Self: 'a;

    fn refs(&self) -> Self::Refs<'_> { (&self.0, self.1.refs()) }
    fn muts(&mut self) -> Self::Muts<'_> { (&mut self.0, self.1.muts()) }
}
