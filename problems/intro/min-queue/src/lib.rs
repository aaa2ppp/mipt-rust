#![forbid(unsafe_code)]

struct Item<T> {
    val: T,
    idx_of_min: usize,
}

pub struct MinStack<T> {
    vec: Vec<Item<T>>
}

impl<T: Clone + Ord> MinStack<T> {
    pub fn new() -> Self {
        Self{ vec: Vec::new() }
    }

    fn vec_push(vec :&mut Vec<Item<T>>, val: T) {
        let mut idx_of_min = vec.len();

        if let Some(idx) = vec.last().map(|it| it.idx_of_min) {
            if val >= vec[idx].val {
                idx_of_min = idx
            }
        }

        vec.push(Item { val, idx_of_min });
    }

    fn vec_min(vec: &Vec<Item<T>>) -> Option<&T>{
        vec.last().map(|it| &vec[it.idx_of_min].val)
    }

    pub fn push(&mut self, val: T) {
        Self::vec_push(&mut self.vec, val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop().map(|it| it.val)
    }

    pub fn last(&mut self) -> Option<&T> {
        self.vec.last().map(|it| &it.val)
    }

    pub fn min(&self) -> Option<&T> {
        Self::vec_min(&self.vec)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn move_to(&mut self, to :&mut MinStack<T>) {
        while let Some(val) = self.pop() {
            to.push(val);
        }
    }
}

pub struct MinQueue<T> {
    input: MinStack<T>,
    output: MinStack<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            input: MinStack::new(),
            output: MinStack::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.input.push(val)
    }

    fn pour(&mut self) {
        if self.output.is_empty() {
            self.input.move_to(&mut self.output);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pour();
        self.output.pop()
    }

    pub fn front(&mut self) -> Option<&T> {
        self.pour();
        self.output.last()
    }

    pub fn min(&self) -> Option<&T> {
        let a = self.input.min();
        let b = self.output.min();

        if a.is_none() {
            b
        } else if b.is_none() {
            a
        } else {
            Some(a.unwrap().min(b.unwrap()))
        }
    }

    pub fn len(&self) -> usize {
        self.input.len() + self.output.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
