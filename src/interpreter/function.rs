use super::backtrace::Backtrace;
use super::context::Context;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::parser::command::Atom;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Function: ToString + Debug {
    fn call(&self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace>;
}

pub struct ScriptFunction {
    pub command: Vec<Atom>,
}

pub type NativeFunctionHandler =
    fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace>;

pub struct NativeFunction {
    pub handler: NativeFunctionHandler,
}

impl ToString for ScriptFunction {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl Debug for ScriptFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Function for ScriptFunction {
    fn call(&self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        let mark = &body.first().unwrap().mark;
        context.slots.clear();
        for atom in body.iter().skip(1) {
            let value = Backtrace::trace(context.resolve_value(atom), mark)?;
            context.slots.push(value);
        }
        context.scopes.push(Object::default());
        let result = Backtrace::trace(context.run_command(&self.command), mark);
        context.scopes.pop();
        result
    }
}

impl ScriptFunction {
    pub fn wrap(command: &[Atom]) -> Value {
        let function: Rc<dyn Function> = Rc::new(ScriptFunction {
            command: command.to_vec(),
        });
        Value::FUNCTION(function)
    }
}

impl ToString for NativeFunction {
    fn to_string(&self) -> String {
        format!("<Native function at {:p}>", self)
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Function for NativeFunction {
    fn call(&self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        (self.handler)(context, body)
    }
}

impl NativeFunction {
    pub fn wrap(handler: NativeFunctionHandler) -> Value {
        let function: Rc<dyn Function> = Rc::new(NativeFunction { handler });
        Value::FUNCTION(function)
    }
}
