use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::closure::Closure;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn closure_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    let value = Closure::new(head.mark.clone(), body.to_vec(), context.scopes.clone());
    Ok(Signal::COMPLETE(Variant::CLOSURE(value)))
}
