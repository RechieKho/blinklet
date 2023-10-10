use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn sub(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let mut sum = context.resolve_number(&body[1])?;

    for atom in body.iter().skip(2) {
        sum -= context.resolve_number(atom)?;
    }

    Ok(Signal::COMPLETE(Variant::NUMBER(sum)))
}
