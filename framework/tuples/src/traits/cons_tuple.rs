use crate::indexes::{Here, Index, There};

mod private {
    pub trait Sealed {}
}

pub trait ConsTuple: private::Sealed {
    type Head;
    type Tail;
    type Length: Index;
}

pub struct Nil;

impl private::Sealed for () {}
impl ConsTuple for () {
    type Head = ();
    type Tail = Nil;
    type Length = Here;
}

impl<Head, Tail: ConsTuple> private::Sealed for (Head, Tail) {}
impl<Head, Tail: ConsTuple> ConsTuple for (Head, Tail) {
    type Head = Head;
    type Tail = Tail;
    type Length = There<Tail::Length>;
}
