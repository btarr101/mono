use crate::index::{Here, Index, There};

mod private {
    pub trait Sealed {}
}

/// Represents a tuple encoded in right-nested cons form.
///
/// # Invariants
/// - `Head` corresponds to the first element of the tuple.
/// - `Tail` is either another cons tuple or `Nil`.
/// - `Length` reflects the total number of elements.
pub trait ConsTuple: private::Sealed {
    /// First element type.
    type Head;
    /// Remaining elements.
    type Tail;
    /// Total element count.
    type Length: Index;
}

/// Marker type representing the empty tail of a cons tuple.
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
