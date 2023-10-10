use super::{represent::Represent, table::Table};
use crate::backtrace::Backtrace;
use crate::interpreter::variant::Variant;
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Scope(HashMap<String, Variant>);

impl Default for Scope {
    fn default() -> Self {
        Scope(HashMap::default())
    }
}

impl Represent for Scope {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(String::from("Scope")) // TODO
    }
}

impl Table for Scope {
    fn insert(&mut self, key: String, value: Variant) -> Option<Variant> {
        self.0.insert(key, value)
    }

    fn remove(&mut self, key: &String) -> Option<Variant> {
        self.0.remove(key)
    }

    fn get<'a>(&'a self, key: &String) -> Option<&'a Variant> {
        self.0.get(key)
    }

    fn contains_key(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }
}

impl Scope {
    pub fn wrap_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Scope::default()))
    }
}
