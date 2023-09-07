use std::sync::Arc;

use super::{object::Object, value::Value};
use crate::parser::command::Atom;

pub trait Function<'code>: ToString {
    fn call(&self, context: &mut Object<'code>, body: &[Atom<'code>]) -> Value<'code>;
}

pub struct ScriptFunction<'code> {
    pub command: Vec<Atom<'code>>,
}

pub type NativeFunctionHandler<'code> =
    fn(context: &mut Object<'code>, body: &[Atom<'code>]) -> Value<'code>;

pub struct NativeFunction<'code> {
    pub handler: NativeFunctionHandler<'code>,
}

impl<'code> ToString for ScriptFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'code> Function<'code> for ScriptFunction<'code> {
    fn call(&self, _context: &mut Object<'code>, _body: &[Atom<'code>]) -> Value<'code> {
        // TODO: Implement this.
        Value::default()
    }
}

impl<'code> ScriptFunction<'code> {
    pub fn wrap(command: &[Atom<'code>]) -> Value<'code> {
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(ScriptFunction {
            command: command.to_vec(),
        });
        Value::FUNCTION(function)
    }
}

impl<'code> ToString for NativeFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Native function at {:p}>", self)
    }
}

impl<'code> Function<'code> for NativeFunction<'code> {
    fn call(&self, context: &mut Object<'code>, body: &[Atom<'code>]) -> Value<'code> {
        (self.handler)(context, body)
    }
}

impl<'code> NativeFunction<'code> {
    pub fn wrap(handler: NativeFunctionHandler<'code>) -> Value<'code> {
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(NativeFunction { handler });
        Value::FUNCTION(function)
    }
}
