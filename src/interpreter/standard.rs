use super::backtrace::Backtrace;
use super::context::Context;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::Atom;

pub fn greet(_context: &mut Context, _body: &[Atom]) -> Result<Signal, Backtrace> {
    println!("Hello world");
    Ok(Signal::COMPLETE(Value::NULL))
}

pub fn error(_context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    println!("Raising error.");
    let mark = body.first().unwrap().mark.clone();
    Err(Backtrace::new(Error {
        message: format!("Error here."),
        mark
    }))
}
