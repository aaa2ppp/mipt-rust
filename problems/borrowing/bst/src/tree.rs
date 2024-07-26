#![forbid(unsafe_code)]
use crate::node::NodeRef;
use std::borrow::Borrow;

pub struct AVLTreeMap<K, V> {
    root: NodeRef<K, V>,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        AVLTreeMap { root: NodeRef::Nil }
    }

    pub fn len(&self) -> usize {
        self.root.size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.root.get(key).val()
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.root.get(key).entry()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        !self.root.get(key).is_nil()
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V>
    where
        K: Ord,
    {
        let old_val;
        (self.root, old_val) = self.root.take().insert(key, val);
        old_val
    }

    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        self.root.get_nth(k).entry()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, val)| val)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let old_entry;
        (self.root, old_entry) = self.root.take().remove(key);
        old_entry
    }
}
