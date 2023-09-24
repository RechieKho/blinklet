use crate::assert_atoms_count;
use crate::atom_as_identifier;
use crate::interpreter::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::log::Log;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::raise_backtrace_bug;
use crate::raise_backtrace_error;

pub fn var(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 3);
    let first_atom = body.first().unwrap();
    let identifier = atom_as_identifier!(&body[1]);
    let value = context.resolve_value(&body[2])?;
    let scope = context.scopes.last_mut();

    if scope.is_none() {
        raise_backtrace_bug!(first_atom.mark.clone(), "Empty scope should be unreachable.");
    }
    let scope = scope.unwrap();
    let popped = scope.content.insert(identifier.clone(), value);
    if popped.is_some() {
        raise_backtrace_error!(first_atom.mark.clone(), "Redeclaration of variable '{}'.", identifier);
    }

    Ok(Signal::COMPLETE(Value::NULL))
}
