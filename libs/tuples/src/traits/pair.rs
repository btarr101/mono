mod private {
    pub trait Sealed {}
}

pub trait Pair: private::Sealed {
    type First;
    type Second;

    fn first(&self) -> &Self::First;
    fn first_mut(&mut self) -> &mut Self::First;
    fn into_first(self) -> Self::First;

    fn second(&self) -> &Self::Second;
    fn second_mut(&mut self) -> &mut Self::Second;
    fn into_second(self) -> Self::Second;
}

impl<A, B> private::Sealed for (A, B) {}
impl<A, B> Pair for (A, B) {
    type First = A;
    type Second = B;

    fn first(&self) -> &Self::First { &self.0 }
    fn first_mut(&mut self) -> &mut Self::First { &mut self.0 }
    fn into_first(self) -> Self::First { self.0 }

    fn second(&self) -> &Self::Second { &self.1 }
    fn second_mut(&mut self) -> &mut Self::Second { &mut self.1 }
    fn into_second(self) -> Self::Second { self.1 }
}
