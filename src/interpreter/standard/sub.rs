use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::parser::command::Atom;

pub fn sub(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let mut sum = context.resolve_number(&body[1])?;

    for atom in body.iter().skip(2) {
        sum -= context.resolve_number(atom)?;
    }

    Ok(Signal::COMPLETE(Value::NUMBER(sum)))
}
