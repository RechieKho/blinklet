use super::function::Function;
use super::object::Object;
use std::sync::Arc;

#[derive(Clone)]
pub enum Value<'code> {
    NULL,
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    OBJECT(Object<'code>),
    FUNCTION(Arc<dyn Function<'code> + 'code>),
}

impl<'code> ToString for Value<'code> {
    fn to_string(&self) -> String {
        match self {
            Value::NULL => String::from("NULL"),
            Value::BOOL(d) => d.to_string(),
            Value::NUMBER(d) => d.to_string(),
            Value::STRING(d) => d.clone(),
            Value::OBJECT(d) => d.to_string(),
            Value::FUNCTION(d) => d.to_string(),
        }
    }
}

impl<'code> Default for Value<'code> {
    fn default() -> Self {
        Value::NULL
    }
}
