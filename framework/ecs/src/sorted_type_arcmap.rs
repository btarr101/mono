use std::{
    any::TypeId,
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    sync::Arc,
};

/// Naming things is really hard...
///
/// Essentially, the reason this exists is because we want to
/// A) Have fast random access lookups for component sets and singleton sets with type ids
/// B) Have a stable sort that can be both deduced at compile and runtime
pub struct SortedTypeArcMap<V: ?Sized> {
    pub direct: HashMap<TypeId, Arc<V>>,
    pub sorted: BTreeMap<TypeId, Arc<V>>,
}

impl<V: ?Sized> Default for SortedTypeArcMap<V> {
    fn default() -> Self { Self::new() }
}

impl<V: ?Sized> SortedTypeArcMap<V> {
    /// Creates a new sorted type arc map
    pub fn new() -> Self {
        Self {
            direct: Default::default(),
            sorted: Default::default(),
        }
    }

    /// Checks if this map has an element
    pub fn has<T: 'static>(&self) -> bool { self.direct.contains_key(&TypeId::of::<T>()) }

    /// Gets an element via direct access
    pub fn get<T: 'static>(&self) -> Option<&Arc<V>> { self.direct.get(&TypeId::of::<T>()) }

    /// Inserts an immediately returns the value inserted
    pub fn insert_and_return<T: 'static>(&mut self, arc: Arc<V>) -> &Arc<V> {
        let key = TypeId::of::<T>();
        let arc = self.direct.entry(key).insert_entry(arc).into_mut();
        self.sorted.insert(key, arc.clone());
        arc
    }

    /// Gets an entry into the sorted type arc map
    pub fn entry<T: 'static>(&mut self) -> SortedTypeArcMapEntry<'_, T, V> {
        match self.has::<T>() {
            true => SortedTypeArcMapEntry::Occupied(self.get::<T>().expect("has")),
            false => SortedTypeArcMapEntry::Vacant(self, PhantomData::<T>),
        }
    }

    /// Iterates over the values in SORTED ORDER
    pub fn values(&self) -> impl Iterator<Item = &Arc<V>> { self.sorted.values() }
}

/// Entry into a sorted type arc map
pub enum SortedTypeArcMapEntry<'a, T, V: ?Sized> {
    Occupied(&'a Arc<V>),
    Vacant(&'a mut SortedTypeArcMap<V>, PhantomData<T>),
}

impl<'a, T: 'static, V: ?Sized> SortedTypeArcMapEntry<'a, T, V> {
    pub fn or_insert_with<F>(self, call: F) -> &'a Arc<V>
    where
        F: FnOnce() -> Arc<V>,
    {
        match self {
            Self::Occupied(arc) => arc,
            Self::Vacant(map, _) => map.insert_and_return::<T>(call()),
        }
    }
}
