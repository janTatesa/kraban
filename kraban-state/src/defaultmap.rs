use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

/// A hashmap that always returns, even if there is no such field. In that case, it returns (and also inserts in get_mut) the default value
#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct DefaultMap<K: Eq + Hash, V: Default> {
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

    pub fn get<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> &V
    where
        K: Borrow<Q>,
    {
        self.inner.get(k).unwrap_or(&self.default)
    }

    pub fn get_mut(&mut self, k: K) -> &mut V {
        self.inner.entry(k).or_default()
    }

    pub fn inner(&self) -> &HashMap<K, V> {
        &self.inner
    }
}
