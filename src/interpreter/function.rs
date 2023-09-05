use std::sync::Arc;

use super::{
    evaluator::EvaluationContext,
    value::{Register, Value},
};
use crate::parser::command::Atom;

pub trait Function<'code>: ToString {
    fn call(&self, context: &mut EvaluationContext<'code>, body: &[Atom<'code>]) -> Value<'code>;
}

pub struct ScriptFunction<'code> {
    pub command: Vec<Atom<'code>>,
}

pub type NativeFunctionHandler<'code> =
    fn(context: &mut EvaluationContext<'code>, body: &[Atom<'code>]) -> Value<'code>;

pub struct NativeFunction<'code> {
    pub handler: NativeFunctionHandler<'code>,
}

impl<'code> ToString for ScriptFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'code> Function<'code> for ScriptFunction<'code> {
    fn call(&self, _context: &mut EvaluationContext<'code>, _body: &[Atom<'code>]) -> Value<'code> {
        // TODO: Implement this.
        Value::default()
    }
}

impl<'code> ScriptFunction<'code> {
    pub fn wrap(command: &[Atom<'code>]) -> Register<'code> {
        let mut register = Register::default();
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(ScriptFunction {
            command: command.to_vec(),
        });
        register.is_constant = true;
        register.value = Value::FUNCTION(function);
        register
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

impl<'code> NativeFunction<'code> {
    pub fn wrap(handler: NativeFunctionHandler<'code>) -> Register<'code> {
        let mut register = Register::default();
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(NativeFunction { handler });
        register.is_constant = true;
        register.value = Value::FUNCTION(function);
        register
    }
}
