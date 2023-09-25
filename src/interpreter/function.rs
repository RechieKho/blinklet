use super::context::Context;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::backtrace::Backtrace;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Function: ToString + Debug {
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
            let value = Backtrace::trace(context.resolve_value(atom), mark)?;
            closure_context.slots.push(value);
        }
        closure_context.scopes.push(Object::default());
        let mut final_result: Result<Signal, Backtrace> = Ok(Signal::COMPLETE(Value::NULL));
        for atom in body.iter().skip(1) {
            if let AtomValue::COMMAND(ref command) = atom.value {
                let result = closure_context.run_command(command.as_slice());
                if result.is_err() {
                    final_result = result;
                    break;
                }
                let signal = result.unwrap();
                if let Signal::RETURN(_) = signal {
                    final_result = Ok(signal);
                    break;
                }
            }
        }
        closure_context.scopes.pop();
        let final_result = Backtrace::trace(final_result, mark);
        final_result
    }
}

impl ScriptFunction {
    pub fn wrap(commands: Vec<Atom>, closure: Vec<Object>) -> Value {
        let function: Rc<dyn Function> = Rc::new(ScriptFunction { commands, closure });
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
