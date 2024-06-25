#![forbid(unsafe_code)]

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Context {
    objs: HashMap<String, Box<dyn Any>>,
    singles: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            objs: HashMap::new(),
            singles: HashMap::new(),
        }
    }

    pub fn insert<K: ToString, V: Any>(&mut self, key: K, obj: V) {
        self.objs.insert(key.to_string(), Box::new(obj));
    }

    pub fn get<T: Any>(&self, key: &str) -> &T {
        self.objs.get(key).unwrap().downcast_ref::<T>().unwrap()
    }

    pub fn insert_singletone<T: Any>(&mut self, obj: T) {
        self.singles.insert(obj.type_id(), Box::new(obj));
    }

    pub fn get_singletone<T: Any>(&self) -> &T {
        self.singles
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref::<T>()
            .unwrap()
    }
}
