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

pub fn set(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 3);
    let first_atom = body.first().unwrap();
    let identifier = atom_as_identifier!(&body[1]);
    let value = context.resolve_value(&body[2])?;

    let scopes_count = context.scopes.len();
    if scopes_count == 0 {
        raise_backtrace_bug!(first_atom.mark.clone(), "Empty scope should be unreachable.");
    }
    for i in (0..scopes_count).rev() {
        let object = context.scopes.get_mut(i);
        if object.is_none() {
            continue;
        }
        let object = object.unwrap();
        if object.content.contains_key(identifier.as_str()) {
            object.content.insert(identifier.clone(), value).unwrap();
            return Ok(Signal::COMPLETE(Value::NULL));
        }
    }

    raise_backtrace_error!(first_atom.mark.clone(), "'{}' is not declared.", identifier);
}
