use crate::assert_atoms_count;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::variant_ops::VariantDuplicate;
use crate::parser::atom::Atom;

pub fn duplicate_fn(
    context: &mut Context,
    head: &Atom,
    body: &[Atom],
) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 1);
    let variant = context.resolve_variant(&body[0])?;
    Ok(Signal::COMPLETE(
        variant.duplicate(Some(head.mark.clone()), context)?,
    ))
}
