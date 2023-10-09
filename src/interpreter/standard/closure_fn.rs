use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::closure::Closure;
use crate::parser::command::Atom;

pub fn closure_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let first_atom = body.first().unwrap();
    let commands = &body[1..];
    let value = Closure::wrap(
        first_atom.mark.clone(),
        commands.to_vec(),
        context.scopes.clone(),
    );
    Ok(Signal::COMPLETE(value))
}
