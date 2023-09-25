use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::function::ScriptFunction;
use crate::interpreter::signal::Signal;
use crate::parser::command::Atom;

pub fn func(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let commands = &body[1..];
    let value = ScriptFunction::wrap(commands.to_vec(), context.scopes.clone());
    Ok(Signal::COMPLETE(value))
}
