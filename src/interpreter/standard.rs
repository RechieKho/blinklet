use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;

pub fn greet<'name, 'code>(
    _context: &mut Context<'name, 'code>,
    _body: &[Atom<'name, 'code>],
) -> Result<Signal<'name, 'code>, Error<'name, 'code>> {
    println!("Hello world");
    Ok(Signal::COMPLETE(Value::NULL))
}
