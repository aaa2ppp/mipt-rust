#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::cmp::max;

pub struct Node<K, V> {
    key: K,
    val: V,
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
    size: usize,
    height: usize,
}

pub enum NodeRef<K, V> {
    Node(Box<Node<K, V>>),
    Nil,
}

impl<K, V> NodeRef<K, V> {
    pub fn new(key: K, val: V) -> Self {
        NodeRef::Node(Box::new(Node {
            key,
            val,
            left: NodeRef::Nil,
            right: NodeRef::Nil,
            size: 1,
            height: 1,
        }))
    }

    // for tests only
    pub fn lnr<'a>(&'a self, mut keys: Vec<&'a K>) -> Vec<&K> {
        if let NodeRef::Node(n) = self {
            keys = n.left.lnr(keys);
            keys.push(&n.key);
            keys = n.right.lnr(keys);
        }
        keys
    }

    // for tests only
    pub fn nlr<'a>(&'a self, mut keys: Vec<&'a K>) -> Vec<&K> {
        if let NodeRef::Node(n) = self {
            keys.push(&n.key);
            keys = n.left.nlr(keys);
            keys = n.right.nlr(keys);
        }
        keys
    }

    // for tests only
    pub fn lrn<'a>(&'a self, mut keys: Vec<&'a K>) -> Vec<&K> {
        if let NodeRef::Node(n) = self {
            keys = n.left.lrn(keys);
            keys = n.right.lrn(keys);
            keys.push(&n.key);
        }
        keys
    }

    pub fn is_nil(&self) -> bool {
        match self {
            NodeRef::Nil => true,
            _ => false,
        }
    }

    pub fn take(&mut self) -> Self {
        let mut n = NodeRef::Nil;
        std::mem::swap(&mut n, self);
        return n;
    }

    fn unwrap(self) -> Box<Node<K, V>> {
        if let NodeRef::Node(n) = self {
            return n;
        } else {
            panic!("can't unwrap node because it Nil")
        }
    }

    pub fn key(&self) -> Option<&K> {
        if let NodeRef::Node(n) = self {
            Some(&n.key)
        } else {
            None
        }
    }

    pub fn val(&self) -> Option<&V> {
        if let NodeRef::Node(n) = self {
            Some(&n.val)
        } else {
            None
        }
    }

    pub fn entry(&self) -> Option<(&K, &V)> {
        if let NodeRef::Node(n) = self {
            Some((&n.key, &n.val))
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        if let NodeRef::Node(n) = self {
            n.size
        } else {
            0
        }
    }

    pub fn height(&self) -> usize {
        if let NodeRef::Node(n) = self {
            n.height
        } else {
            0
        }
    }

    pub fn get<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let NodeRef::Node(n) = self {
            if key < n.key.borrow() {
                n.left.get(key)
            } else if key > n.key.borrow() {
                n.right.get(key)
            } else {
                self
            }
        } else {
            &NodeRef::Nil
        }
    }

    pub fn get_nth(&self, i: usize) -> &Self {
        if let NodeRef::Node(n) = self {
            let left_size = n.left.size();
            if i < left_size {
                n.left.get_nth(i)
            } else if i > left_size {
                n.right.get_nth(i - left_size - 1)
            } else {
                self
            }
        } else {
            &NodeRef::Nil
        }
    }

    pub fn insert(self, key: K, val: V) -> (Self, Option<V>)
    where
        K: Ord,
    {
        if let NodeRef::Node(mut n) = self {
            if key < n.key {
                let (new_left, old_val) = n.left.insert(key, val);
                n.left = new_left;
                (NodeRef::repair(n), old_val)
            } else if key > n.key {
                let (new_right, old_val) = n.right.insert(key, val);
                n.right = new_right;
                (NodeRef::repair(n), old_val)
            } else {
                let old_val = n.val;
                n.val = val;
                (NodeRef::Node(n), Some(old_val))
            }
        } else {
            (NodeRef::new(key, val), None)
        }
    }

    pub fn remove<Q>(self, key: &Q) -> (Self, Option<(K, V)>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let NodeRef::Node(mut n) = self {
            if key < n.key.borrow() {
                let (new_left, old_node) = n.left.remove(key);
                n.left = new_left;
                return (NodeRef::repair(n), old_node);
            }

            if key > n.key.borrow() {
                let (new_right, old_node) = n.right.remove(key);
                n.right = new_right;
                return (NodeRef::repair(n), old_node);
            }

            let (entry, left, right) = (Some((n.key, n.val)), n.left, n.right);

            if left.is_nil() {
                return (right, entry);
            }

            if let NodeRef::Node(n) = right {
                let (new_right, mut min_node) = NodeRef::remove_min(n);
                (min_node.left, min_node.right) = (left, new_right);
                return (NodeRef::repair(min_node), entry);
            } else {
                return (left, entry);
            }
        } else {
            (NodeRef::Nil, None)
        }
    }

    fn remove_min(mut n: Box<Node<K, V>>) -> (Self, Box<Node<K, V>>) {
        if let NodeRef::Node(left) = n.left {
            let (new_left, min_node) = NodeRef::remove_min(left);
            n.left = new_left;
            (NodeRef::repair(n), min_node)
        } else {
            let right = n.right;
            n.right = NodeRef::Nil;
            NodeRef::update(&mut n);
            (right, n)
        }
    }

    fn update(n: &mut Box<Node<K, V>>) {
        n.size = n.left.size() + n.right.size() + 1;
        n.height = max(n.left.height(), n.right.height()) + 1;
    }

    fn repair(mut n: Box<Node<K, V>>) -> Self {
        let (left_height, right_height) = (n.left.height(), n.right.height());

        let d = (left_height as isize) - (right_height as isize);
        if d < -1 {
            NodeRef::left_rotate(n)
        } else if d > 1 {
            NodeRef::right_rotate(n)
        } else {
            NodeRef::update(&mut n);
            NodeRef::Node(n)
        }
    }

    fn left_rotate(mut al: Box<Node<K, V>>) -> Self {
        let mut bt = al.right.unwrap();

        if bt.right.height() > bt.left.height() {
            al.right = bt.left;
            NodeRef::update(&mut al);
            bt.left = NodeRef::Node(al);
            NodeRef::update(&mut bt);
            return NodeRef::Node(bt);
        } else {
            let mut ga = bt.left.unwrap();
            al.right = ga.left;
            NodeRef::update(&mut al);
            bt.left = ga.right;
            NodeRef::update(&mut bt);
            ga.left = NodeRef::Node(al);
            ga.right = NodeRef::Node(bt);
            NodeRef::update(&mut ga);
            return NodeRef::Node(ga);
        }
    }

    fn right_rotate(mut al: Box<Node<K, V>>) -> Self {
        let mut bt = al.left.unwrap();

        if bt.left.height() > bt.right.height() {
            al.left = bt.right;
            NodeRef::update(&mut al);
            bt.right = NodeRef::Node(al);
            NodeRef::update(&mut bt);
            return NodeRef::Node(bt);
        } else {
            let mut ga = bt.right.unwrap();
            al.left = ga.right;
            NodeRef::update(&mut al);
            bt.right = ga.left;
            NodeRef::update(&mut bt);
            ga.right = NodeRef::Node(al);
            ga.left = NodeRef::Node(bt);
            NodeRef::update(&mut ga);
            return NodeRef::Node(ga);
        }
    }
}
