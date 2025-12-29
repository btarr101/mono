use pastey::paste;

use crate::impl_tuple_trait;

/// Trait for prepending elements to any tuple
pub trait CanPrepend: private::Sealed {
    type Prepended<Head>: CanPrepend;

    fn prepend<Head>(self, head: Head) -> Self::Prepended<Head>;
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for () {}
impl CanPrepend for () {
    type Prepended<Head> = (Head,);

    fn prepend<Head>(self, head: Head) -> Self::Prepended<Head> { (head,) }
}

macro_rules! impl_prepend_tuple {
    (@recur) => {};
	// Hack, since we cannot prepend indefinitely, so if we attempt to prepend past
	// the limit nothing happens.
	($first:ident;$idx:tt $(,$rest:ident;$idxs:tt)*) => {
		impl_prepend_tuple!(@munch [] $first;$idx $(,$rest;$idxs)*);

		impl<$first $(,$rest)*> private::Sealed for ($first, $($rest,)*) {}
		impl<$first $(,$rest)*> CanPrepend for ($first, $($rest,)*) {
			type Prepended<Head> = Self;

			fn prepend<Head>(self, _: Head) -> Self::Prepended<Head> {
				self
			}
		}
	};
	(@recur $first:ident;$idx:tt $(,$rest:ident;$idxs:tt)*) => {
		impl_prepend_tuple!(@munch [] $first;$idx $(,$rest;$idxs)*);

		impl<$first $(,$rest)*> private::Sealed for ($first, $($rest,)*) {}
		impl<$first $(,$rest)*> CanPrepend for ($first, $($rest,)*) {
			type Prepended<Head> = (Head, $first, $($rest,)*);

			fn prepend<Head>(self, head: Head) -> Self::Prepended<Head> {
				paste! {
					let (e0, $([<e $idxs>],)*) = self;
					(head, e0, $([<e $idxs>],)*)
				}
			}
		}
	};
	(@munch [$($first:ident;$idxs:tt),*] $last:ident;$idx:tt ) => {
		impl_prepend_tuple!(@recur $($first;$idxs),*);
	};
	(@munch [$($first:ident;$idxs:tt),*] $next:ident;$idx:tt, $($rest:ident;$rest_idxs:tt),+ ) => {
		impl_prepend_tuple!(@munch [$($first;$idxs,)* $next;$idx] $($rest;$rest_idxs),+);
	};
}

impl_tuple_trait!(impl_prepend_tuple);
