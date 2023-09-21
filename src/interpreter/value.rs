use super::function::Function;
use super::object::Object;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value<'name, 'code> {
    NULL,
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    LIST(Vec<Value<'name, 'code>>),
    OBJECT(Object<'name, 'code>),
    FUNCTION(Rc<dyn Function<'name, 'code> + 'code>),
}

impl<'name, 'code> ToString for Value<'name, 'code> {
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

impl<'name, 'code> Default for Value<'name, 'code> {
    fn default() -> Self {
        Value::NULL
    }
}
