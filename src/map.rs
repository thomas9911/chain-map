use std::borrow::Borrow;

#[derive(Debug, PartialEq)]
pub enum OwnedOrBorrowed<'a, V> {
    /// most of the time remote data is owned
    Owned(V),
    /// most of the time local data is borrowed
    Borrowed(&'a V),
}

impl<'a, V: Clone> Clone for OwnedOrBorrowed<'a, V> {
    fn clone(&self) -> Self {
        OwnedOrBorrowed::Owned(self.as_owned())
    }
}

impl<'a, V> std::ops::Deref for OwnedOrBorrowed<'a, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        match self {
            OwnedOrBorrowed::Borrowed(v) => v,
            OwnedOrBorrowed::Owned(v) => &v,
        }
    }
}

impl<'a, V: Clone> OwnedOrBorrowed<'a, V> {
    pub fn as_owned(&self) -> V {
        match self {
            OwnedOrBorrowed::Borrowed(v) => v.clone().clone(),
            OwnedOrBorrowed::Owned(v) => v.clone(),
        }
    }
}

impl<'a, V> OwnedOrBorrowed<'a, V> {
    pub fn ref_inner(&'a self) -> &'a V {
        match self {
            OwnedOrBorrowed::Borrowed(v) => v,
            OwnedOrBorrowed::Owned(v) => &v,
        }
    }
}

pub trait Map<K, V> {
    fn get<R: Borrow<K>>(&self, key: R) -> Option<OwnedOrBorrowed<V>>;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn remove<R: Borrow<K>>(&mut self, k: R) -> Option<V>;
}
