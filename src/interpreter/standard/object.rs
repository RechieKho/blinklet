use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::object::Object;
use crate::interpreter::signal::Signal;
use crate::parser::command::Atom;
use crate::raise_error;

pub fn object(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let scope = Object::default();
    let signal = context.run_commands(&body[1..], scope)?;
    match signal {
        Signal::BREAK(ref mark) | Signal::CONTINUE(ref mark) => {
            raise_error!(Some(mark.clone()), "Loop control structure is forbidden.");
        },
        Signal::COMPLETE(value) | Signal::RETURN(value, _) => {
            Ok(Signal::COMPLETE(value))
        }
    }
}
