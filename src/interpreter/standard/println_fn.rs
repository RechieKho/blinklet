use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::represent::Represent;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn println_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    for atom in body.iter() {
        let value = context.resolve_variant(atom)?;
        print!("{}", value.represent(Some(head.mark.clone()))?);
    }
    print!("\n");
    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
