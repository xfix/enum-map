use EnumMap;
use Internal;

use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::iter::Extend;
use core::ops::{Index, IndexMut};

impl<K: Internal<V> + Debug, V: Debug> Debug for EnumMap<K, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<K: Internal<V>, V> Extend<(K, V)> for EnumMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self[key] = value;
        }
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for EnumMap<K, V>
where
    K: Internal<V> + Copy,
    V: Copy,
{
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K: Internal<V>, V> Index<K> for EnumMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        &self.as_slice()[key.to_usize()]
    }
}

impl<K: Internal<V>, V> IndexMut<K> for EnumMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        &mut self.as_mut_slice()[key.to_usize()]
    }
}

// Implementations provided by derive attribute are too specific, and put requirements on K.
// This is caused by rust-lang/rust#26925.
impl<K: Internal<V>, V> Clone for EnumMap<K, V>
where
    K::Array: Clone,
{
    fn clone(&self) -> Self {
        EnumMap {
            array: self.array.clone(),
        }
    }
}

impl<K: Internal<V>, V> Copy for EnumMap<K, V>
where
    K::Array: Copy,
{
}

impl<K: Internal<V>, V: PartialEq> PartialEq for EnumMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<K: Internal<V>, V: Eq> Eq for EnumMap<K, V> {}

impl<K: Internal<V>, V: Hash> Hash for EnumMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<K: Internal<V>, V: Default> Default for EnumMap<K, V> {
    fn default() -> Self {
        enum_map! { _ => V::default() }
    }
}
