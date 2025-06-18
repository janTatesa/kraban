use std::{borrow::Borrow, hash::Hash};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// A hashmap that always returns, even if there is no such field. In that case, it returns (and also inserts in get_mut) the default value
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct DefaultMap<K: Eq + Hash, V> {
    inner: HashMap<K, V>,
    #[serde(skip)]
    default: V,
}

impl<K: Eq + Hash, V: Default> DefaultMap<K, V> {
    pub fn new(hash_map: HashMap<K, V>) -> Self {
        Self {
            inner: hash_map,
            default: V::default(),
        }
    }

    pub fn get<Q>(&self, k: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.inner.get(k).unwrap_or(&self.default)
    }

    pub fn get_mut<'a, Q>(&mut self, k: &'a Q) -> &mut V
    where
        K: Borrow<Q>,
        &'a Q: Into<K>,
        Q: ToOwned<Owned = K> + Hash + Eq + ?Sized,
    {
        self.inner.entry_ref(k).or_default()
    }

    pub fn inner(&self) -> &HashMap<K, V> {
        &self.inner
    }
}
