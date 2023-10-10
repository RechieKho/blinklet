use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::parser::atom::Atom;

pub fn add(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let mut sum = 0f64;

    for atom in body.iter().skip(1) {
        sum += context.resolve_number(atom)?;
    }

    Ok(Signal::COMPLETE(Value::NUMBER(sum)))
}
