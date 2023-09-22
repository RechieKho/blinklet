use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;

pub fn greet(_context: &mut Context, _body: &[Atom]) -> Result<Signal, Error> {
    println!("Hello world");
    Ok(Signal::COMPLETE(Value::NULL))
}
