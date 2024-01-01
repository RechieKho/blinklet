use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::represent::Represent;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count, raise_error};

pub fn assert_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 2);
    let success = context.resolve_boolean(&body[0])?;
    if !success.is_true() {
        raise_error!(
            Some(body[0].mark.clone()),
            "{}",
            context
                .resolve_variant(&body[1])?
                .represent(Some(body[1].mark.clone()))?
        );
    }
    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
