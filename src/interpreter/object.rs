use super::standard::add::add;
use super::standard::break_fn::break_fn;
use super::standard::continue_fn::continue_fn;
use super::standard::cs::cs;
use super::standard::div::div;
use super::standard::list::list;
use super::standard::mul::mul;
use super::standard::object::object as create_object;
use super::standard::print::print;
use super::standard::rep::rep;
use super::standard::return_fn::return_fn;
use super::standard::set::set;
use super::standard::sub::sub;
use super::standard::var::var;
use super::value::Value;
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};

macro_rules! object_register_function {
    ($object:expr, $function:expr) => {{
        $object.content.insert(
            String::from(stringify!($function)),
            Value::FUNCTION(Arc::new($function)),
        );
    }};

    ($object:expr, $string:expr, $function:expr) => {{
        $object
            .content
            .insert(String::from($string), Value::FUNCTION(Arc::new($function)));
    }};
}

#[derive(Debug, Clone)]
pub struct Object {
    pub content: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        let mut object = Object {
            content: HashMap::default(),
        };
        object_register_function!(object, var);
        object_register_function!(object, set);
        object_register_function!(object, print);
        object_register_function!(object, list);
        object_register_function!(object, rep);
        object_register_function!(object, add);
        object_register_function!(object, sub);
        object_register_function!(object, mul);
        object_register_function!(object, div);
        object_register_function!(object, cs);
        object_register_function!(object, "object", create_object);
        object_register_function!(object, "return", return_fn);
        object_register_function!(object, "break", break_fn);
        object_register_function!(object, "continue", continue_fn);
        object
    }

    pub fn with_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Object::new()))
    }
}
