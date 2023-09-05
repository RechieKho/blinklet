use super::{function::Function, object::Object};
use std::sync::Arc;

#[derive(Clone)]
pub enum Value<'code> {
    NULL,
    NUMBER(f64),
    STRING(String),
    OBJECT(Object<'code>),
    FUNCTION(Arc<dyn Function<'code> + 'code>),
}

impl<'code> ToString for Value<'code> {
    fn to_string(&self) -> String {
        match self {
            Value::NULL => String::from("NULL"),
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

#[derive(Clone)]
pub struct Register<'code> {
    pub value: Value<'code>,
    pub is_constant: bool,
}

impl<'code> Default for Register<'code> {
    fn default() -> Self {
        Register {
            value: Value::NULL,
            is_constant: false,
        }
    }
}

impl<'code> From<Value<'code>> for Register<'code> {
    fn from(value: Value<'code>) -> Self {
        Register {
            value,
            is_constant: false,
        }
    }
}
