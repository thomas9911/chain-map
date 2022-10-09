use crate::{Map, OwnedOrBorrowed};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

macro_rules! impl_map_trait_inner {
    ($struct:ty) => {
        fn get<R: Borrow<K>>(&self, key: R) -> Option<OwnedOrBorrowed<V>> {
            <$struct>::get(&self, key.borrow()).map(OwnedOrBorrowed::Borrowed)
        }

        fn insert(&mut self, k: K, v: V) -> Option<V> {
            <$struct>::insert(self, k, v)
        }

        fn remove<R: Borrow<K>>(&mut self, key: R) -> Option<V> {
            <$struct>::remove(self, key.borrow())
        }
    };
}

macro_rules! impl_map_trait {
    ($struct:ty, $constrained:tt $(+ $constrained_extra:tt )*, $constrained_value:tt $(+ $constrained_value_extra:tt )*) => {
        impl<K: $constrained $(+ $constrained_extra )*, V: $constrained_value $(+ $constrained_value_extra )*> Map<K, V> for $struct {
            impl_map_trait_inner!($struct);
        }
    };
    ($struct:ty, $constrained:tt $(+ $constrained_extra:tt )*) => {
        impl<K: $constrained $(+ $constrained_extra )*, V> Map<K, V> for $struct {
            impl_map_trait_inner!($struct);
        }
    };
}

impl_map_trait!(HashMap<K, V>, Eq + Hash);
impl_map_trait!(BTreeMap<K, V>, Ord);

impl<K: PartialEq, V> Map<K, V> for Vec<(K, V)> {
    fn get<R: Borrow<K>>(&self, key: R) -> Option<OwnedOrBorrowed<V>> {
        self.iter()
            .find(|(k, _)| k == key.borrow())
            .map(|(_, v)| OwnedOrBorrowed::Borrowed(v))
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        let res = Map::remove(self, &k);
        self.push((k, v));
        res
    }

    fn remove<R: Borrow<K>>(&mut self, k: R) -> Option<V> {
        self.retain(|(key, _)| key != k.borrow());
        None
    }
}

#[cfg(feature = "im")]
impl_map_trait!(im::HashMap<K, V>, Eq + Hash + Clone, Clone);
#[cfg(feature = "im")]
impl_map_trait!(im::OrdMap<K, V>, Ord + Clone, Clone);

#[cfg(feature = "indexmap")]
impl_map_trait!(indexmap::IndexMap<K, V>, Eq + Hash);

#[cfg(feature = "radix_trie")]
use radix_trie::TrieKey;
#[cfg(feature = "radix_trie")]
impl_map_trait!(radix_trie::Trie<K, V>, TrieKey);

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash, V: Clone> Map<K, V> for dashmap::DashMap<K, V> {
    fn get<'a, R: Borrow<K>>(&'a self, key: R) -> Option<OwnedOrBorrowed<V>> {
        dashmap::DashMap::get(&self, key.borrow())
            .map(|r| OwnedOrBorrowed::Owned(r.value().clone()))
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        dashmap::DashMap::insert(self, k, v)
    }

    fn remove<R: Borrow<K>>(&mut self, key: R) -> Option<V> {
        dashmap::DashMap::remove(self, key.borrow()).map(|(_, v)| v)
    }
}

#[cfg(feature = "sled")]
impl<K: AsRef<[u8]>> Map<K, sled::IVec> for sled::Tree {
    fn get<Q: Borrow<K>>(&self, key: Q) -> Option<OwnedOrBorrowed<sled::IVec>> {
        sled::Tree::get(self, key.borrow())
            .ok()
            .flatten()
            .map(OwnedOrBorrowed::Owned)
    }

    fn insert(&mut self, k: K, v: sled::IVec) -> Option<sled::IVec> {
        match sled::Tree::insert(self, k, &v) {
            Ok(x) => x,
            Err(_) => Some(v),
        }
    }

    fn remove<R: Borrow<K>>(&mut self, k: R) -> Option<sled::IVec> {
        sled::Tree::remove(self, k.borrow()).ok().flatten()
    }
}

// #[test]
// #[cfg(feature = "sled")]
// fn sled_test() {
//     let db: sled::Db = sled::open("my_db").unwrap();
//     let mut tree = db.open_tree("__sled_default").unwrap();

//     // assert_eq!(None, Map::<_, _>::get::<&[u8]>(&tree, &sled::IVec::from(b"yo!")));
//     assert_eq!(None, Map::<&[u8], _>::get(&tree, "yo!"));
//     assert_eq!(None, Map::insert(&mut tree, b"yo!", b"v1".into()));

//     db.drop_tree("__sled_default").unwrap();
// }
