use super::function::NativeFunction;
use super::standard::greet;
use super::standard::log;
use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object {
    pub content: HashMap<String, Value>,
}

impl ToString for Object {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl Default for Object {
    fn default() -> Self {
        let mut object = Object {
            content: HashMap::default(),
        };
        object
            .content
            .insert(String::from("greet"), NativeFunction::wrap(greet));

        object
            .content
            .insert(String::from("log"), NativeFunction::wrap(log));
        object
    }
}
