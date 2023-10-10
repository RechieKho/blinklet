use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::list::List;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn list_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let mut values: Vec<Variant> = Vec::new();

    for atom in body.iter().skip(1) {
        let value = context.resolve_value(atom)?;
        values.push(value);
    }

    Ok(Signal::COMPLETE(Variant::LIST(List::from(values))))
}
