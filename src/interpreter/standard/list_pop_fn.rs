use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;

pub fn list_pop_fn(
    context: &mut Context,
    _head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 2);
    let mut list = context.resolve_list(&body[0])?;
    let value = list.pop(Some(body[0].mark.clone()))?;
    Ok(Signal::COMPLETE(value))
}
