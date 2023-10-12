use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::raise_bug;
use crate::{assert_atoms_count_min, atom_as_identifier};

pub fn list_pop(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let first_atom = body.first().unwrap();
    let mut list = context.resolve_list(&body[1])?;
    let table = context.scopes.last_mut();

    if table.is_none() {
        raise_bug!(
            Some(first_atom.mark.clone()),
            "Empty table should be unreachable."
        );
    }

    let table = table.unwrap();

    for atom in body.iter().skip(2) {
        let identifier = atom_as_identifier!(atom);
        table.insert(identifier.clone(), list.pop()?, Some(atom.mark.clone()))?;
    }

    Ok(Signal::COMPLETE(Variant::LIST(list)))
}
