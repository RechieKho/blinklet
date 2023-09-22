use super::function::Function;
use super::object::Object;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    NULL,
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    LIST(Vec<Value>),
    OBJECT(Object),
    FUNCTION(Rc<dyn Function>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::NULL => String::from("NULL"),
            Value::BOOL(d) => d.to_string(),
            Value::NUMBER(d) => d.to_string(),
            Value::STRING(d) => d.clone(),
            Value::OBJECT(d) => d.to_string(),
            Value::FUNCTION(d) => d.to_string(),
            Value::LIST(d) => format!(
                "[{}]",
                d.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::NULL
    }
}
