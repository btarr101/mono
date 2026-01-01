/// Trait for iterating a homogenous cons tuple
pub trait ConsIter<V> {
    /// Converts this into an iterator
    fn into_iter(self) -> impl Iterator<Item = V>;
}

impl<V> ConsIter<V> for () {
    fn into_iter(self) -> impl Iterator<Item = V> { std::iter::empty() }
}

impl<Head, Tail> ConsIter<Head> for (Head, Tail)
where
    Tail: ConsIter<Head>,
{
    fn into_iter(self) -> impl Iterator<Item = Head> { std::iter::once(self.0).chain(self.1.into_iter()) }
}
