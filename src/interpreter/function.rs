use crate::parser::command::{Command, Atom};
use super::{value::Value, evaluator::EvaluationContext};

pub trait Function<'code> : ToString {
    fn call(&self, context: &mut EvaluationContext<'code>, body: &[Atom<'code>]) -> Value<'code>;
}

pub struct ScriptFunction<'code> {
    pub command : Command<'code>
}

pub type NativeFunctionHandler<'code> = fn(context: &mut EvaluationContext<'code>, body: &[Atom<'code>]) -> Value<'code>;

pub struct NativeFunction<'code> {
    pub handler : NativeFunctionHandler<'code>
}

impl<'code> ToString for ScriptFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'code> Function<'code> for ScriptFunction<'code> {
    fn call(&self, _context: &mut EvaluationContext<'code>, _body: &[Atom<'code>]) -> Value<'code> {
        // TODO: Implement this.
        Value::NULL
    }
}

impl<'code> ToString for NativeFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Native function at {:p}>", self)
    }
}

impl<'code> Function<'code> for NativeFunction<'code> {
    fn call(&self, context: &mut EvaluationContext<'code>, body: &[Atom<'code>]) -> Value<'code> {
        (self.handler)(context, body)
    }
}
