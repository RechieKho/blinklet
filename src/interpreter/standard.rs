use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;

pub fn greet<'code>(
    _context: &mut Context<'code>,
    _body: &[Atom<'code>],
) -> Result<Signal<'code>, Error<'code>> {
    println!("Hello world");
    Ok(Signal::COMPLETE(Value::NULL))
}
