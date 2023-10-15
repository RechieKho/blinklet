use crate::assert_atoms_count;
use crate::atom_as_identifier;
use crate::backtrace::Backtrace;
use crate::context_get_current_scope;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::raise_error;

pub fn var_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 2);
    let identifier = atom_as_identifier!(&body[0]);
    let variant = context.resolve_variant(&body[1])?;
    let table = context_get_current_scope!(context);
    let popped = table.insert(identifier.clone(), variant, Some(head.mark.clone()))?;
    if popped.is_some() {
        raise_error!(
            Some(head.mark.clone()),
            "Redeclaration of variable '{}'.",
            identifier
        );
    }
    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
