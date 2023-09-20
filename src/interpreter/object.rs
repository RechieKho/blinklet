use super::function::NativeFunction;
use super::standard::greet;
use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Object<'code> {
    pub content: HashMap<String, Value<'code>>,
}

impl<'code> ToString for Object<'code> {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl<'code> Default for Object<'code> {
    fn default() -> Self {
        let mut object = Object {
            content: HashMap::default(),
        };
        object
            .content
            .insert(String::from("greet"), NativeFunction::wrap(greet));
        object
    }
}
