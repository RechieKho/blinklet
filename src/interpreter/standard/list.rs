use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::parser::command::Atom;

pub fn list(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let mut values: Vec<Value> = Vec::new();

    for atom in body.iter().skip(1) {
        let value = context.resolve_value(atom)?;
        values.push(value);
    }

    Ok(Signal::COMPLETE(Value::LIST(values)))
}
