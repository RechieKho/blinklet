use crate::parser::command::Atom;

use super::{evaluator::EvaluationContext, value::Value};

pub fn greet(_context: &mut EvaluationContext, _body : &[Atom]) -> Value {
    println!("Hello world");
    Value::NULL
}
