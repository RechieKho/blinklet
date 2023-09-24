use super::function::NativeFunction;
use super::standard::print::print;
use super::standard::set::set;
use super::standard::var::var;
use super::value::Value;
use std::collections::HashMap;

macro_rules! object_register_native_function {
    ($object:expr, $function:expr) => {
        $object.content.insert(
            String::from(stringify!($function)),
            NativeFunction::wrap($function),
        )
    };
}

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
        object_register_native_function!(object, var);
        object_register_native_function!(object, set);
        object_register_native_function!(object, print);
        object
    }
}
