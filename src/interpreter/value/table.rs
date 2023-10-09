use super::{represent::Represent, Value};

pub trait Table: Represent {
    fn insert(&mut self, key: String, value: Value) -> Option<Value>;
    fn remove(&mut self, key: &String) -> Option<Value>;
    fn get<'a>(&'a self, key: &String) -> Option<&'a Value>;
    fn contains_key(&self, key: &String) -> bool;
}
