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

pub fn set_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, Some(head.mark.clone()), 2);
    let identifier = atom_as_identifier!(&body[0]);
    let value = context.resolve_variant(&body[1])?;

    let scopes_count = context.scopes.len();
    if scopes_count == 0 {
        raise_bug!(
            Some(head.mark.clone()),
            "Empty scopes should be unreachable."
        );
    }
    for i in (0..scopes_count).rev() {
        let table = context.scopes.get_mut(i).unwrap();
        if table.contains_key(identifier, Some(head.mark.clone()))? {
            table
                .insert(identifier.clone(), value, Some(head.mark.clone()))
                .unwrap();
            return Ok(Signal::COMPLETE(Variant::NULL(Null())));
        }
    }

    raise_error!(Some(head.mark.clone()), "'{}' is not declared.", identifier);
}
