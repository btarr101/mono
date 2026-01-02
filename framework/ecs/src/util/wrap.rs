use derived_deref::{Deref, DerefMut};

/// I honestly myself don't even know why I need this...
#[derive(Deref, DerefMut)]
pub struct Wrap<T>(pub T);
