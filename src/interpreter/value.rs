use super::function::Function;
use super::object::Object;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value<'code> {
    NULL,
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    LIST(Vec<Value<'code>>),
    OBJECT(Object<'code>),
    FUNCTION(Rc<dyn Function<'code> + 'code>),
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

impl<'code> Default for Value<'code> {
    fn default() -> Self {
        Value::NULL
    }
}
