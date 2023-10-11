use crate::assert_atoms_count_max;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn return_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_max!(body, 2);
    let mark = &body.first().unwrap().mark;
    if body.len() == 1 {
        Ok(Signal::RETURN(Variant::NULL(Null()), mark.clone()))
    } else {
        let value = context.resolve_variant(&body[1])?;
        Ok(Signal::RETURN(value, mark.clone()))
    }
}
