mod built_ins;
mod chain_map;
mod map;

pub use chain_map::ChainedMap;
pub use map::{Map, OwnedOrBorrowed};

#[cfg(test)]
use std::collections::HashMap;

#[test]
fn test_get() {
    let mut test = HashMap::new();
    test.insert("test", "oke");

    assert_eq!(Some("oke"), Map::get(&test, "test").map(|x| x.as_owned()));
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

    assert_eq!(Some("test"), test.get("nested").map(|x| x.as_owned()));
    assert_eq!(Some("yes"), test.get("root").map(|x| x.as_owned()));
    assert_eq!(None, test.get("other"));
}

#[test]
fn test_delete() {
    let mut test = HashMap::new();
    test.insert("test", "oke");

    assert_eq!(Some("oke"), Map::get(&test, "test").map(|x| x.as_owned()));
    assert_eq!(Some("oke"), Map::remove(&mut test, "test"));
    assert_eq!(None, Map::get(&test, "other"));
}

#[test]
fn test_chained_overlapping_get() {
    let mut other = HashMap::new();
    other.insert("one", "nested");

    let mut test = ChainedMap::new();
    test.insert("one", "base");
    test.chain(other);

    assert_eq!(Some("base"), test.get("one").map(|x| x.as_owned()));
    assert_eq!(
        Some("nested"),
        test.get_from_other("one").map(|x| x.as_owned())
    );
}

#[test]
fn test_get_propagate() {
    let mut other = HashMap::new();
    other.insert("two", "hallo");

    let mut test = ChainedMap::new();
    test.chain(other);

    // normal get does not alter data
    assert_eq!(Some("hallo"), test.get("two").map(|x| x.as_owned()));

    assert_eq!(None, test.get_from_root("two"));
    assert_eq!(
        Some("hallo"),
        test.get_from_other("two").map(|x| x.as_owned())
    );

    // get propagate does alter data
    assert_eq!(
        Some("hallo"),
        test.get_propagate("two").map(|x| x.as_owned())
    );

    assert_eq!(
        Some("hallo"),
        test.get_from_root("two").map(|x| x.as_owned())
    );
    assert_eq!(
        Some("hallo"),
        test.get_from_other("two").map(|x| x.as_owned())
    );
}
