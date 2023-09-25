use crate::assert_atoms_count;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::object::Object;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::parser::command::Atom;

pub fn object(_context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 1);
    let value = Value::OBJECT(Object::default());
    Ok(Signal::COMPLETE(value))
}
