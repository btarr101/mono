/// Applies a tuple-implementing macro over supported arities.
///
/// Tuples up to 16 elements are supported. Traits generated with this macro
/// that cannot express arbitrarily long tuples (such as `CanPrepend`) become
/// no-ops at the maximum arity.
#[macro_export]
macro_rules! impl_tuple_trait {
    ($macro:ident) => {
        $macro!(
            A;0, B;1, C;2, D;3, E;4, F;5, G;6, H;7, I;8, J;9, K;10, L;11, M;12, N;13, O;14, P;15, Q;16, R;17, S;18
        );
    };
}
