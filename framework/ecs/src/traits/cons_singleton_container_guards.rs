mod private {
    pub trait Sealed {}
}

// Trait for a tuple set of singleton containers
// pub trait ConsSingletonContainerGuards: private::Sealed + Sized {
//     type MaybeLockedGuards: ConsMaybeLockedGuardsExt<LockedGuards = Self>;
//     fn get_maybe_locks(world: &World) -> Self::MaybeLockedGuards;
// }
