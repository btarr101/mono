/// Converts a cons tuple to a cons tuple of refs
pub trait ConsAsRefTuple: private::Sealed {
    /// Cons tuple as refs
    type AsRefs<'a>: 'a
    where
        Self: 'a;

    /// Cons tuple as muts
    type AsMuts<'a>: 'a
    where
        Self: 'a;

    /// Gets a cons tuple of refs
    fn as_refs(&self) -> Self::AsRefs<'_>;

    /// Gets a cons tuple of mutable refs
    fn as_muts(&mut self) -> Self::AsMuts<'_>;
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for () {}
impl ConsAsRefTuple for () {
    type AsRefs<'a> = ();
    type AsMuts<'a> = ();

    fn as_refs(&self) -> Self::AsRefs<'_> {}
    fn as_muts(&mut self) -> Self::AsMuts<'_> {}
}

impl<Head, Tail> private::Sealed for (Head, Tail) {}
impl<Head, Tail> ConsAsRefTuple for (Head, Tail)
where
    Tail: ConsAsRefTuple,
{
    type AsRefs<'a>
        = (&'a Head, Tail::AsRefs<'a>)
    where
        Self: 'a;
    type AsMuts<'a>
        = (&'a mut Head, Tail::AsMuts<'a>)
    where
        Self: 'a;

    fn as_refs(&self) -> Self::AsRefs<'_> { (&self.0, self.1.as_refs()) }
    fn as_muts(&mut self) -> Self::AsMuts<'_> { (&mut self.0, self.1.as_muts()) }
}
