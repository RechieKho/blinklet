use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::null::Null;
use crate::interpreter::value::represent::Represent;
use crate::interpreter::value::Value;
use crate::parser::atom::Atom;

pub fn print(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    for atom in body.iter().skip(1) {
        let value = context.resolve_value(atom)?;
        print!("{}", value.represent()?);
    }
    Ok(Signal::COMPLETE(Value::NULL(Null())))
}
