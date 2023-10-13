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

pub fn set_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 3);
    let first_atom = body.first().unwrap();
    let identifier = atom_as_identifier!(&body[1]);
    let value = context.resolve_variant(&body[2])?;

    let scopes_count = context.scopes.len();
    if scopes_count == 0 {
        raise_bug!(
            Some(first_atom.mark.clone()),
            "Empty table should be unreachable."
        );
    }
    for i in (0..scopes_count).rev() {
        let table = context.scopes.get_mut(i);
        if table.is_none() {
            continue;
        }
        let table = table.unwrap();
        if table.contains_key(identifier, Some(first_atom.mark.clone()))? {
            table
                .insert(identifier.clone(), value, Some(first_atom.mark.clone()))
                .unwrap();
            return Ok(Signal::COMPLETE(Variant::NULL(Null())));
        }
    }

    raise_error!(
        Some(first_atom.mark.clone()),
        "'{}' is not declared.",
        identifier
    );
}
