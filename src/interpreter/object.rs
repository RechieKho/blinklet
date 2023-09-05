use std::{collections::HashMap, sync::Arc};
use super::{value::Value, function::{Function, NativeFunction, NativeFunctionHandler}, standard::greet};

#[derive(Clone)]
pub struct Object<'code> {
    pub content : HashMap<String, Value<'code>>
}

impl<'code> ToString for Object<'code> {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl<'code> Default for Object<'code> {
    fn default() -> Object<'code> {
        let mut object = Object { content: HashMap::default() };
        object.content.insert(String::from("greet"), NativeFunction::wrap(greet));
        object
    }

}
