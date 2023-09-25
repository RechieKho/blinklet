use crate::assert_atoms_count_max;
use crate::raise_error;
use crate::log::Log;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::parser::command::Atom;

pub fn return_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_max!(body, 2);
    if body.len() == 1 {
        Ok(Signal::RETURN(Value::NULL))
    } else {
        let value = context.resolve_value(&body[1])?;
        Ok(Signal::RETURN(value))
    }
}
