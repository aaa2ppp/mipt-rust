#![forbid(unsafe_code)]

use std::cmp::max;
use std::vec::Vec;

pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    size: usize,
    height: usize,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            size: 1,
            height: 1,
        }
    }

    pub fn lnr<'a>(n: Option<&'a Box<Self>>, mut keys: Vec<&'a K>) -> Vec<&'a K> {
        if let Some(n) = n {
            keys = Node::lnr(n.left.as_ref(), keys);
            keys.push(&n.key);
            keys = Node::lnr(n.right.as_ref(), keys);
        }
        keys
    }

    pub fn nlr<'a>(n: Option<&'a Box<Self>>, mut keys: Vec<&'a K>) -> Vec<&'a K> {
        if let Some(n) = n {
            keys.push(&n.key);
            keys = Node::nlr(n.left.as_ref(), keys);
            keys = Node::nlr(n.right.as_ref(), keys);
        }
        keys
    }

    pub fn lrn<'a>(n: Option<&'a Box<Self>>, mut keys: Vec<&'a K>) -> Vec<&'a K> {
        if let Some(n) = n {
            keys = Node::lrn(n.left.as_ref(), keys);
            keys = Node::lrn(n.right.as_ref(), keys);
            keys.push(&n.key);
        }
        keys
    }

    pub fn key(n: Option<&Box<Self>>) -> Option<&K> {
        n.map(|n| &n.key)
    }

    pub fn val(n: Option<&Box<Self>>) -> Option<&V> {
        n.map(|n| &n.val)
    }

    pub fn entry(n: Option<&Box<Self>>) -> Option<(&K, &V)> {
        n.map(|n| (&n.key, &n.val))
    }

    pub fn size(n: Option<&Box<Self>>) -> usize {
        n.map_or(0, |n| n.size)
    }

    pub fn height(n: Option<&Box<Self>>) -> usize {
        n.map_or(0, |n| n.height)
    }

    pub fn get<'a>(n: Option<&'a Box<Self>>, key: &K) -> Option<&'a Box<Self>> {
        // ??? 'a
        if let Some(n) = n {
            if *key < n.key {
                Node::get(n.left.as_ref(), key)
            } else if *key > n.key {
                Node::get(n.right.as_ref(), key)
            } else {
                Some(n)
            }
        } else {
            None
        }
    }

    pub fn get_nth(n: Option<&Box<Self>>, i: usize) -> Option<&Box<Self>> {
        if let Some(n) = n {
            let left_size = Node::size(n.left.as_ref());
            if i < left_size {
                Node::get_nth(n.left.as_ref(), i)
            } else if i > left_size {
                Node::get_nth(n.right.as_ref(), i - left_size - 1)
            } else {
                Some(n)
            }
        } else {
            None
        }
    }

    pub fn insert(n: Option<Box<Self>>, key: K, val: V) -> (Option<Box<Self>>, Option<V>) {
        if let Some(mut n) = n {
            if key < n.key {
                let (new_left, old_val) = Node::insert(n.left, key, val);
                n.left = new_left;
                (Some(Node::repair(n)), old_val)
            } else if key > n.key {
                let (new_right, old_val) = Node::insert(n.right, key, val);
                n.right = new_right;
                (Some(Node::repair(n)), old_val)
            } else {
                let old_val = n.val;
                n.val = val;
                (Some(n), Some(old_val))
            }
        } else {
            (Some(Box::new(Node::new(key, val))), None)
        }
    }

    pub fn remove(n: Option<Box<Self>>, key: &K) -> (Option<Box<Self>>, Option<Box<Self>>) {
        if let Some(mut n) = n {
            if *key < n.key {
                let (new_left, old_node) = Node::remove(n.left, key);
                n.left = new_left;
                return (Some(Node::repair(n)), old_node);
            }

            if *key > n.key {
                let (new_right, old_node) = Node::remove(n.right, key);
                n.right = new_right;
                return (Some(Node::repair(n)), old_node);
            }

            let (left, right) = Node::untie(&mut n);

            if left.is_none() {
                return (right, Some(n));
            }

            if right.is_none() {
                return (left, Some(n));
            }

            let (new_right, mut min_node) = Node::remove_min(right.unwrap());
            min_node.left = left;
            min_node.right = new_right;
            Node::update(&mut min_node);
            (Some(min_node), Some(n))
        } else {
            (None, None)
        }
    }

    fn remove_min(mut n: Box<Self>) -> (Option<Box<Self>>, Box<Self>) {
        if n.left.is_none() {
            let (_, right) = Node::untie(&mut n);
            return (right, n);
        }

        let (new_left, min_node) = Node::remove_min(n.left.unwrap());
        n.left = new_left;
        (Some(Node::repair(n)), min_node)
    }

    fn untie(n: &mut Box<Self>) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let (left, right) = (n.left.take(), n.right.take());
        n.left = None;
        n.right = None;
        Node::update(n);
        (left, right)
    }

    fn update(n: &mut Box<Self>) {
        n.size = Node::size(n.left.as_ref()) + Node::size(n.right.as_ref()) + 1;
        n.height = max(
            Node::height(n.left.as_ref()),
            Node::height(n.right.as_ref()),
        ) + 1;
    }

    fn repair(mut n: Box<Self>) -> Box<Self> {
        let (left_height, right_height) = (
            Node::height(n.left.as_ref()),
            Node::height(n.right.as_ref()),
        );
        let d = (left_height as isize) - (right_height as isize);
        if d < -1 {
            Node::left_rotate(n)
        } else if d > 1 {
            Node::right_rotate(n)
        } else {
            Node::update(&mut n);
            n
        }
    }

    fn left_rotate(mut al: Box<Self>) -> Box<Self> {
        // let mut al = n;
        let mut bt = al.right.unwrap();

        let (bt_left_height, bt_right_height) = (
            Node::height(bt.left.as_ref()),
            Node::height(bt.right.as_ref()),
        );

        if bt_right_height > bt_left_height {
            al.right = bt.left;
            Node::update(&mut al);
            bt.left = Some(al);
            Node::update(&mut bt);
            return bt;
        } else {
            let mut ga = bt.left.unwrap();
            al.right = ga.left;
            Node::update(&mut al);
            bt.left = ga.right;
            Node::update(&mut bt);
            ga.left = Some(al);
            ga.right = Some(bt);
            Node::update(&mut ga);
            return ga;
        }
    }

    fn right_rotate(mut al: Box<Self>) -> Box<Self> {
        // let mut al = n;
        let mut bt = al.left.unwrap();

        let (bt_left_height, bt_right_height) = (
            Node::height(bt.left.as_ref()),
            Node::height(bt.right.as_ref()),
        );

        if bt_left_height > bt_right_height {
            al.left = bt.right;
            Node::update(&mut al);
            bt.right = Some(al);
            Node::update(&mut bt);
            return bt;
        } else {
            let mut ga = bt.right.unwrap();
            al.left = ga.right;
            Node::update(&mut al);
            bt.right = ga.left;
            Node::update(&mut bt);
            ga.right = Some(al);
            ga.left = Some(bt);
            Node::update(&mut ga);
            return ga;
        }
    }
}
