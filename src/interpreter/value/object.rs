use crate::interpreter::value::Value;
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Object {
    pub content: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            content: HashMap::default(),
        }
    }

    pub fn with_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Object::new()))
    }
}
