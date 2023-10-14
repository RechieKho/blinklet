use crate::assert_atoms_count;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;

pub fn list_length_fn(
    context: &mut Context,
    head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, Some(head.mark.clone()), 1);
    let list = context.resolve_list(&body[0])?;
    Ok(Signal::COMPLETE(list.length(Some(body[0].mark.clone()))?))
}
