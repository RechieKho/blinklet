use crate::assert_atoms_count;
use crate::atom_as_identifier;
use crate::interpreter::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::log::Log;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::raise_backtrace_error;

pub fn var(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 3..3);
    let identifier = atom_as_identifier!(&body[1]);
    let value = context.resolve_value(&body[2])?;
    context.declare(identifier.clone(), value);
    Ok(Signal::COMPLETE(Value::NULL))
}
