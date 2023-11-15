use crate::assert_atoms_count;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;

pub fn list_get_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 2);
    let list = context.resolve_list(&body[0])?;
    let index = context.resolve_float(&body[1])?;
    Ok(Signal::COMPLETE(list.get(index, Some(head.mark.clone()))?))
}
