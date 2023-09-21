use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;
use std::rc::Rc;

pub trait Function<'name, 'code>: ToString
where
    'name: 'code,
{
    fn call(
        &self,
        context: &mut Context<'name, 'code>,
        body: &[Atom<'name, 'code>],
    ) -> Result<Signal<'name, 'code>, Error<'name, 'code>>;
}

pub struct ScriptFunction<'name, 'code> {
    pub command: Vec<Atom<'name, 'code>>,
}

pub type NativeFunctionHandler<'name, 'code> =
    fn(
        context: &mut Context<'name, 'code>,
        body: &[Atom<'name, 'code>],
    ) -> Result<Signal<'name, 'code>, Error<'name, 'code>>;

pub struct NativeFunction<'name, 'code> {
    pub handler: NativeFunctionHandler<'name, 'code>,
}

impl<'name, 'code> ToString for ScriptFunction<'name, 'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'name, 'code> Function<'name, 'code> for ScriptFunction<'name, 'code>
where
    'name: 'code,
{
    fn call(
        &self,
        context: &mut Context<'name, 'code>,
        body: &[Atom<'name, 'code>],
    ) -> Result<Signal<'name, 'code>, Error<'name, 'code>> {
        for atom in body.iter() {
            let value = context.resolve_value(atom)?;
            context.slots.push(value);
        }

        context.run_command(&self.command)
    }
}

impl<'name, 'code> ScriptFunction<'name, 'code>
where
    'name: 'code,
{
    pub fn wrap(command: &[Atom<'name, 'code>]) -> Value<'name, 'code> {
        let function: Rc<dyn Function<'name, 'code> + 'code> = Rc::new(ScriptFunction {
            command: command.to_vec(),
        });
        Value::FUNCTION(function)
    }
}

impl<'name, 'code> ToString for NativeFunction<'name, 'code> {
    fn to_string(&self) -> String {
        format!("<Native function at {:p}>", self)
    }
}

impl<'name, 'code> Function<'name, 'code> for NativeFunction<'name, 'code>
where
    'name: 'code,
{
    fn call(
        &self,
        context: &mut Context<'name, 'code>,
        body: &[Atom<'name, 'code>],
    ) -> Result<Signal<'name, 'code>, Error<'name, 'code>> {
        (self.handler)(context, body)
    }
}

impl<'name, 'code> NativeFunction<'name, 'code>
where
    'name: 'code,
{
    pub fn wrap(handler: NativeFunctionHandler<'name, 'code>) -> Value<'name, 'code> {
        let function: Rc<dyn Function<'name, 'code> + 'code> = Rc::new(NativeFunction { handler });
        Value::FUNCTION(function)
    }
}
