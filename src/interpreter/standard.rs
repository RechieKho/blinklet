use crate::parser::command::Atom;

use super::{object::Object, value::Value};

pub fn greet<'code>(_context: &mut Object, _body: &[Atom<'code>]) -> Value<'code> {
    println!("Hello world");
    Value::NULL
}
