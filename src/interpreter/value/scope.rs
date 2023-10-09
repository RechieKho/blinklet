use crate::interpreter::value::Value;
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};
use super::table::Table;

#[derive(Debug, Clone)]
pub struct Scope (HashMap<String, Value>);

impl Default for Scope {
    fn default() -> Self {
        Scope(HashMap::default())
    }
}

impl Table for Scope {
    fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    fn remove(&mut self, key: &String) -> Option<Value> {
        self.0.remove(key)
    }

    fn get<'a>(&'a self, key: &String) -> Option<&'a Value> {
        self.0.get(key)
    }

    fn contains_key(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }
}

impl Scope {
    pub fn with_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Scope::default()))
    }
}
