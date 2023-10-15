use crate::assert_atoms_count;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;

pub fn continue_fn(
    _context: &mut Context,
    head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 0);
    Ok(Signal::CONTINUE(head.mark.clone()))
}
