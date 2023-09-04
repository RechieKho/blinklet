use std::{collections::HashMap, sync::Arc};
use super::{object::Object, function::Function};

#[derive(Clone)]
pub enum Value {
    NULL,
    NUMBER(f64),
    STRING(String),
    OBJECT(Object),
    FUNCTION(Arc<dyn Function>)
}

pub type ValueMap = HashMap<String, Value>;

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::NULL => String::from("NULL"),
            Value::NUMBER(d) => d.to_string(),
            Value::STRING(d) => d.clone(),
            Value::OBJECT(d) => d.to_string(),
            Value::FUNCTION(d) => d.to_string()
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::NULL
    }
}
