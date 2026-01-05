/// Applies a tuple-implementing macro over supported arities.
#[macro_export]
macro_rules! impl_tuple_trait {
    ($macro:ident) => {
        $macro!(A;0, B;1, C;2, D;3, E;4, F;5);
    };
}
