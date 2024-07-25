#![forbid(unsafe_code)]
use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        AVLTreeMap { root: None }
    }

    pub fn len(&self) -> usize {
        Node::size(self.root.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Node::val(Node::get(self.root.as_ref(), key))
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        Node::entry(Node::get(self.root.as_ref(), key))
    }

    pub fn contains_key(&self, key: &K) -> bool {
        !Node::get(self.root.as_ref(), key).is_none()
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let (new_root, old_val) = Node::insert(self.root.take(), key, val);
        self.root = new_root;
        old_val
    }

    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        Node::entry(Node::get_nth(self.root.as_ref(), k))
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let (new_root, old_node) = Node::remove(self.root.take(), key);
        self.root = new_root;
        old_node.map(|n| n.val)
    }

    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let (new_root, old_node) = Node::remove(self.root.take(), key);
        self.root = new_root;
        old_node.map(|n| (n.key, n.val))
    }
}
