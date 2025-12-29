use crate::impl_tuple_trait;

/// Trait for a tuple that can be converted to a cons style tuple.
pub trait AsConsTuple: private::Sealed {
    type As;
    type AsRefs<'a>: 'a
    where
        Self: 'a;
    type AsMuts<'a>: 'a
    where
        Self: 'a;

    /// Converts this tuple into a cons-style tuple
    fn to_cons_tuple(self) -> Self::As;

    /// Converts this tuple into a cons-style tuple of refs
    fn to_cons_ref_tuple(&self) -> Self::AsRefs<'_>;

    /// Converts this tuple into a cons-style tuple of mutable refs
    fn to_cons_mut_tuple(&mut self) -> Self::AsMuts<'_>;
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for () {}
impl AsConsTuple for () {
    type As = ();
    type AsRefs<'a> = ();
    type AsMuts<'a> = ();

    fn to_cons_tuple(self) -> Self::As {}
    fn to_cons_ref_tuple(&self) -> Self::AsRefs<'_> {}
    fn to_cons_mut_tuple(&mut self) -> Self::AsMuts<'_> {}
}

macro_rules! impl_as_cons_tuple {
    () => {};
	($first:ident;$idx:tt $(,$rest:ident;$idxs:tt)*) => {
		impl_as_cons_tuple!(@munch [] $first;$idx $(,$rest;$idxs)*);

		impl<$first $(,$rest)*> private::Sealed for ($first, $($rest,)*) {}
		impl<$first $(,$rest)*> AsConsTuple for ($first, $($rest,)*) {
			type As = ($first, <($($rest,)*) as AsConsTuple>::As);
			type AsRefs<'a> = (&'a $first, <($(&'a $rest,)*) as AsConsTuple>::As) where Self: 'a;
			type AsMuts<'a> = (&'a mut $first, <($(&'a mut $rest,)*) as AsConsTuple>::As) where Self: 'a;

			fn to_cons_tuple(self) -> Self::As {
				let head = self.$idx;
				let tail = ($(self.$idxs,)*).to_cons_tuple();

				(head, tail)
			}

			fn to_cons_ref_tuple(&self) -> Self::AsRefs<'_> {
				let head = &self.$idx;
				let tail = ($(&self.$idxs,)*).to_cons_tuple();

				(head, tail)
			}

			fn to_cons_mut_tuple(&mut self) -> Self::AsMuts<'_> {
				let head = &mut self.$idx;
				let tail = ($(&mut self.$idxs,)*).to_cons_tuple();

				(head, tail)
			}
		}
	};
	(@munch [$($first:ident;$idxs:tt),*] $last:ident;$idx:tt ) => {
		impl_as_cons_tuple!($($first;$idxs),*);
	};
	(@munch [$($first:ident;$idxs:tt),*] $next:ident;$idx:tt, $($rest:ident;$rest_idxs:tt),+ ) => {
		impl_as_cons_tuple!(@munch [$($first;$idxs,)* $next;$idx] $($rest;$rest_idxs),+);
	};
}

impl_tuple_trait!(impl_as_cons_tuple);
