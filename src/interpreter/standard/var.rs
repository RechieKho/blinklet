use crate::assert_atoms_count;
use crate::atom_as_identifier;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::raise_bug;
use crate::raise_error;

pub fn var(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 3);
    let first_atom = body.first().unwrap();
    let identifier = atom_as_identifier!(&body[1]);
    let value = context.resolve_variant(&body[2])?;
    let table = context.scopes.last_mut();

    if table.is_none() {
        raise_bug!(
            Some(first_atom.mark.clone()),
            "Empty table should be unreachable."
        );
    }
    let table = table.unwrap();
    let popped = table.insert(identifier.clone(), value, Some(first_atom.mark.clone()))?;
    if popped.is_some() {
        raise_error!(
            Some(first_atom.mark.clone()),
            "Redeclaration of variable '{}'.",
            identifier
        );
    }

    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
