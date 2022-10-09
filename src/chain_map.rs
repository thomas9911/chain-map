use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{Map, OwnedOrBorrowed};

pub struct ChainedMap<K, V, O: Map<K, V>> {
    data: HashMap<K, V>,
    other: Option<O>,
}

impl<K, V, O: Map<K, V>> ChainedMap<K, V, O> {
    pub fn new() -> Self {
        ChainedMap {
            data: HashMap::new(),
            other: None,
        }
    }

    pub fn chain(&mut self, other: O) {
        self.other = Some(other)
    }

    pub fn get_from_other<R: Borrow<K>>(&self, key: R) -> Option<OwnedOrBorrowed<V>> {
        if let Some(other) = self.other.as_ref() {
            other.get(key.borrow())
        } else {
            None
        }
    }
}

impl<K, V, O> ChainedMap<K, V, O>
where
    K: Eq + Hash,
    O: Map<K, V>,
{
    pub fn get_from_root<R: Borrow<K>>(&self, key: R) -> Option<OwnedOrBorrowed<V>> {
        self.data.get(key.borrow()).map(OwnedOrBorrowed::Borrowed)
    }
}

impl<K, V, O> ChainedMap<K, V, O>
where
    K: Eq + Hash + Clone,
    V: Clone,
    O: Map<K, V>,
{
    /// get possible nested value but also clone it into this map
    pub fn get_propagate<'a, R: Borrow<K> + Clone>(
        &'a mut self,
        key: R,
    ) -> Option<OwnedOrBorrowed<V>> {
        let is_nested = self.other.is_some();

        if is_nested {
            let other = self.other.as_ref().unwrap();
            if let Some(value) = other.get(key.clone()) {
                self.data.insert(key.borrow().clone(), value.as_owned());
                Some(value)
            } else {
                None
            }
        } else {
            if let Some(value) = self.data.get(key.borrow()) {
                Some(value).map(OwnedOrBorrowed::Borrowed)
            } else {
                None
            }
        }
    }
}

impl<K: Eq + Hash, V, O: Map<K, V>> Map<K, V> for ChainedMap<K, V, O> {
    fn get<Q: Borrow<K>>(&self, key: Q) -> Option<OwnedOrBorrowed<V>> {
        if let Some(value) = self.data.get(key.borrow()) {
            return Some(OwnedOrBorrowed::Borrowed(value));
        }

        if let Some(other) = &self.other {
            other.get(key)
        } else {
            None
        }
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.data.insert(k, v)
    }

    fn remove<Q: Borrow<K>>(&mut self, k: Q) -> Option<V> {
        self.data.remove(k.borrow())
    }
}
