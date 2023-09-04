use crate::parser::command::{Command, Atom};
use super::{value::Value, evaluator::EvaluationContext};

pub trait Function : ToString {
    fn call(&self, context: &mut EvaluationContext, body: &[Atom]) -> Value;
}

pub struct ScriptFunction<'a> {
    pub name : String,
    pub command : Command<'a>
}

pub struct NativeFunction {
    pub name : String,
    pub handler : fn(context: &mut EvaluationContext, body: &[Atom]) -> Value
}

impl<'a> ToString for ScriptFunction<'a> {
    fn to_string(&self) -> String {
        format!("<Function '{}'>", self.name)
    }
}

impl<'a> Function for ScriptFunction<'a> {
    fn call(&self, _context: &mut EvaluationContext, _body: &[Atom]) -> Value {
        // TODO: Implement this.
        Value::NULL
    }
}

impl ToString for NativeFunction {
    fn to_string(&self) -> String {
        format!("<Function '{}'>", self.name)
    }
}

impl Function for NativeFunction {
    fn call(&self, context: &mut EvaluationContext, body: &[Atom]) -> Value {
        (self.handler)(context, body)
    }
}
