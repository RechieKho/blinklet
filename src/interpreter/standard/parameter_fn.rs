use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::{atom_as_identifier, context_get_current_scope, raise_error};

pub fn parameter_fn(
    context: &mut Context,
    head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    let table = context_get_current_scope!(context, Some(head.mark.clone()));
    for atom in body.iter() {
        let identifier = atom_as_identifier!(atom);
        let argument = context.slots.pop();
        match argument {
            Option::None => {
                raise_error!(
                    Some(atom.mark.clone()),
                    "Arguments supplied is insufficient for '{}'",
                    identifier
                );
            }
            Option::Some(variant) => {
                let variant = table.insert(identifier.clone(), variant, Some(head.mark.clone()))?;
                if variant.is_some() {
                    raise_error!(
                        Some(atom.mark.clone()),
                        "'{}' has already been defined.",
                        identifier
                    );
                }
            }
        }
    }

    if context.slots.len() != 0 {
        raise_error!(
            Some(head.mark.clone()),
            "Excess arguments are passed, remaining {} argument(s).",
            context.slots.len()
        );
    }

    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
