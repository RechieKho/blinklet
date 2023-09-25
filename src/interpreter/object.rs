use super::function::NativeFunction;
use super::standard::add::add;
use super::standard::break_fn::break_fn;
use super::standard::continue_fn::continue_fn;
use super::standard::div::div;
use super::standard::list::list;
use super::standard::mul::mul;
use super::standard::print::print;
use super::standard::rep::rep;
use super::standard::return_fn::return_fn;
use super::standard::set::set;
use super::standard::sub::sub;
use super::standard::var::var;
use super::value::Value;
use hashbrown::HashMap;

macro_rules! object_register_native_function {
    ($object:expr, $function:expr) => {
        $object.content.insert(
            String::from(stringify!($function)),
            NativeFunction::wrap($function),
        )
    };

    ($object:expr, $string:expr, $function:expr) => {
        $object
            .content
            .insert(String::from($string), NativeFunction::wrap($function))
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
        object_register_native_function!(object, list);
        object_register_native_function!(object, rep);
        object_register_native_function!(object, add);
        object_register_native_function!(object, sub);
        object_register_native_function!(object, mul);
        object_register_native_function!(object, div);
        object_register_native_function!(object, "return", return_fn);
        object_register_native_function!(object, "break", break_fn);
        object_register_native_function!(object, "continue", continue_fn);
        object
    }
}
