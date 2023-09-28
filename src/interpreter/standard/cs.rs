use crate::backtrace::Backtrace;
use crate::interpreter::closure::Closure;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::command::Atom;

pub fn cs(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let commands = &body[1..];
    let value = Closure::wrap(commands.to_vec(), context.scopes.clone());
    Ok(Signal::COMPLETE(value))
}
