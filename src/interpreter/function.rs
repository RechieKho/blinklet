use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;
use std::rc::Rc;

pub trait Function<'code>: ToString {
    fn call(
        &self,
        context: &mut Context<'code>,
        body: &[Atom<'code>],
    ) -> Result<Signal<'code>, Error<'code>>;
}

pub struct ScriptFunction<'code> {
    pub command: Vec<Atom<'code>>,
}

pub type NativeFunctionHandler<'code> =
    fn(context: &mut Context<'code>, body: &[Atom<'code>]) -> Result<Signal<'code>, Error<'code>>;

pub struct NativeFunction<'code> {
    pub handler: NativeFunctionHandler<'code>,
}

impl<'code> ToString for ScriptFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'code> Function<'code> for ScriptFunction<'code> {
    fn call(
        &self,
        context: &mut Context<'code>,
        body: &[Atom<'code>],
    ) -> Result<Signal<'code>, Error<'code>> {
        for atom in body.iter() {
            let value = context.resolve_value(atom)?;
            context.slots.push(value);
        }

        context.run_command(&self.command)
    }
}

impl<'code> ScriptFunction<'code> {
    pub fn wrap(command: &[Atom<'code>]) -> Value<'code> {
        let function: Rc<dyn Function<'code> + 'code> = Rc::new(ScriptFunction {
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
    fn call(
        &self,
        context: &mut Context<'code>,
        body: &[Atom<'code>],
    ) -> Result<Signal<'code>, Error<'code>> {
        (self.handler)(context, body)
    }
}

impl<'code> NativeFunction<'code> {
    pub fn wrap(handler: NativeFunctionHandler<'code>) -> Value<'code> {
        let function: Rc<dyn Function<'code> + 'code> = Rc::new(NativeFunction { handler });
        Value::FUNCTION(function)
    }
}
