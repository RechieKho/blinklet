use super::context::Context;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::backtrace::Backtrace;
use crate::parser::command::Atom;
use crate::raise_error;
use crate::signal_no_loop_control;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Function: ToString + Debug + Sync + Send{
    fn call(&self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace>;
}

pub struct ScriptFunction {
    pub commands: Vec<Atom>,
    pub closure: Vec<Object>,
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
        let mut closure_context = Context::new(self.closure.clone(), Vec::new());
        for atom in body.iter().skip(1) {
            let value = context.resolve_value(atom)?;
            closure_context.slots.push(value);
        }

        let signal = closure_context.run_commands(&self.commands, Object::default())?;
        signal_no_loop_control!(signal);
        return Ok(signal);
    }
}

impl ScriptFunction {
    pub fn wrap(commands: Vec<Atom>, closure: Vec<Object>) -> Value {
        let function: Arc<dyn Function> = Arc::new(ScriptFunction { commands, closure });
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
