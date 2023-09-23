use crate::interpreter::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::log::Log;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;

pub fn var(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    if body.len() >= 4 {
        let unexpected_atom = &body[4];
        return Err(Backtrace::new(Log::error(format!("Unexpected word."), unexpected_atom.mark.clone())));
    }

    let identifier = if let AtomValue::IDENTIFIER(ref identifier) = &body[1].value {
        identifier
    } else {
        return Err(Backtrace::new(Log::error(format!("Expecting an identifier."), body[1].mark.clone())));
    };

    let value = context.resolve_value(&body[2])?;

    let scope = context.scopes.last_mut().unwrap();
    scope.content.insert(identifier.clone(), value);

    Ok(Signal::COMPLETE(Value::NULL))
}
