use crate::assert_atoms_count_max;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn return_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_max!(body, 1);
    if body.len() == 1 {
        let value = context.resolve_variant(&body[0])?;
        Ok(Signal::RETURN(value, head.mark.clone()))
    } else {
        Ok(Signal::RETURN(Variant::NULL(Null()), head.mark.clone()))
    }
}
