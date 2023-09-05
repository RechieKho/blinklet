use crate::parser::command::Atom;

use super::{evaluator::EvaluationContext, value::Value};

pub fn greet<'code>(_context: &mut EvaluationContext<'code>, _body : &[Atom<'code>]) -> Value<'code> {
    println!("Hello world");
    Value::NULL
}
