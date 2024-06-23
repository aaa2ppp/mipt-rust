#![forbid(unsafe_code)]
use std::rc::Rc;

struct Node<T> {
    value: T,
    prev: Option<Rc<Node<T>>>,
}

pub struct PRef<T> {
    node: Rc<Node<T>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node.value
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct PStack<T> {
    top: Option<Rc<Node<T>>>,
    len: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self { top: None, len: 0 }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        Self {
            top: self.top.clone(),
            len: self.len,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, value: T) -> Self {
        let node = Rc::new(Node {
            prev: self.top.clone(),
            value,
        });
        Self {
            top: Some(node),
            len: self.len + 1,
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        self.top.clone().map(|node| {
            let tail = Self {
                top: node.prev.clone(),
                len: self.len - 1,
            };
            (PRef { node }, tail)
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> PStackIterator<T> {
        PStackIterator {
            stack: self.clone(),
        }
    }
}

pub struct PStackIterator<T> {
    stack: PStack<T>,
}

impl<T> Iterator for PStackIterator<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some((top, tail)) = self.stack.pop() {
            self.stack = tail;
            return Some(top);
        }
        None
    }
}
