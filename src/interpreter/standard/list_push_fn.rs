use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn list_push_fn(
    context: &mut Context,
    head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 2);
    let mut list = context.resolve_list(&body[0])?;
    for atom in body.iter().skip(1) {
        let element = context.resolve_variant(atom)?;
        list.push(element, Some(head.mark.clone()))?;
    }
    Ok(Signal::COMPLETE(Variant::LIST(list)))
}
