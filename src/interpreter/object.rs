use super::function::NativeFunction;
use super::standard::greet;
use super::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Object<'name, 'code> {
    pub content: HashMap<String, Value<'name, 'code>>,
}

impl<'name, 'code> ToString for Object<'name, 'code> {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl<'name, 'code> Default for Object<'name, 'code>
where
    'name: 'code,
{
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
