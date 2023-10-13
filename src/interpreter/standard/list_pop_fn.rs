use crate::backtrace::Backtrace;
use crate::context_get_current_scope;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count_min, atom_as_identifier};

pub fn list_pop_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, Some(head.mark.clone()), 2);
    let mut list = context.resolve_list(&body[0])?;
    let table = context_get_current_scope!(context, Some(head.mark.clone()));
    for atom in body.iter().skip(1) {
        let identifier = atom_as_identifier!(atom);
        table.insert(identifier.clone(), list.pop()?, Some(atom.mark.clone()))?;
    }
    Ok(Signal::COMPLETE(Variant::LIST(list)))
}
