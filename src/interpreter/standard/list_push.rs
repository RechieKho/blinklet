use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn list_push(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let mut list = context.resolve_list(&body[1])?;

    for atom in body.iter().skip(2) {
        let element = context.resolve_variant(atom)?;
        list.push(element)?;
    }

    Ok(Signal::COMPLETE(Variant::LIST(list)))
}
