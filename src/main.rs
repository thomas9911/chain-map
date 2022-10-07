use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

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

    pub fn get_from_other<R: Borrow<K>>(&self, key: R) -> Option<&V> {
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
    pub fn get_from_root<R: Borrow<K>>(&self, key: R) -> Option<&V> {
        self.data.get(key.borrow())
    }
}

impl<K, V, O> ChainedMap<K, V, O>
where
    K: Eq + Hash + Clone,
    V: Clone,
    O: Map<K, V>,
{
    /// 
    pub fn get_propagate<'a, R: Borrow<K> + Clone>(&'a mut self, key: R) -> Option<&'a V> {
        let is_nested = self.other.is_some();

        if is_nested {
            let other = self.other.as_ref().unwrap();
            if let Some(value) = other.get(key.clone()) {
                self.data.insert(key.borrow().clone(), value.clone());
                Some(value)
            } else {
                None
            }
        } else {
            if let Some(value) = self.data.get(key.borrow()) {
                Some(value)
            } else {
                None
            }
        }
    }
}

pub trait Map<K, V> {
    fn get<R: Borrow<K>>(&self, key: R) -> Option<&V>;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

impl<K: Eq + Hash, V> Map<K, V> for HashMap<K, V> {
    fn get<R: Borrow<K>>(&self, key: R) -> Option<&V> {
        HashMap::get(&self, key.borrow())
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        HashMap::insert(self, k, v)
    }
}

impl<K: Eq + Hash, V, O: Map<K, V>> Map<K, V> for ChainedMap<K, V, O> {
    fn get<Q: Borrow<K>>(&self, key: Q) -> Option<&V> {
        if let Some(value) = self.data.get(key.borrow()) {
            return Some(value);
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
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_get() {
    let mut test = HashMap::new();
    test.insert("test", "oke");

    assert_eq!(Some(&"oke"), Map::get(&test, "test"));
    assert_eq!(Some(&"oke"), HashMap::get(&test, "test"));
    assert_eq!(None, Map::get(&test, "other"));
}

#[test]
fn test_chained_get() {
    let mut other = HashMap::new();
    other.insert("nested", "test");

    let mut test = ChainedMap::new();
    test.insert("root", "yes");
    test.chain(other);

    assert_eq!(Some(&"test"), test.get("nested"));
    assert_eq!(Some(&"yes"), test.get("root"));
    assert_eq!(None, test.get("other"));
}

#[test]
fn test_chained_overlapping_get() {
    let mut other = HashMap::new();
    other.insert("one", "nested");

    let mut test = ChainedMap::new();
    test.insert("one", "base");
    test.chain(other);

    assert_eq!(Some(&"base"), test.get("one"));
    assert_eq!(Some(&"nested"), test.get_from_other("one"));
}

#[test]
fn test_get_propagate() {
    let mut other = HashMap::new();
    other.insert("two", "hallo");

    let mut test = ChainedMap::new();
    test.chain(other);

    // normal get does not alter data
    assert_eq!(Some(&"hallo"), test.get("two"));

    assert_eq!(None, test.get_from_root("two"));
    assert_eq!(Some(&"hallo"), test.get_from_other("two"));

    // get propagate does alter data
    assert_eq!(Some(&"hallo"), test.get_propagate("two"));

    assert_eq!(Some(&"hallo"), test.get_from_root("two"));
    assert_eq!(Some(&"hallo"), test.get_from_other("two"));
}
