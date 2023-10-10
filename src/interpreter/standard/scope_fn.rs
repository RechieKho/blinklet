use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::scope::Scope;
use crate::parser::atom::Atom;

pub fn scope_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let scope = Scope::wrap_arc_mutex();
    context.run_commands(&body[1..], scope)
}
