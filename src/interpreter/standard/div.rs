use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::variant_ops::VariantDiv;
use crate::parser::atom::Atom;

pub fn div(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let mut variant = context.resolve_variant(&body[1])?;

    for atom in body.iter().skip(2) {
        let rhs = context.resolve_variant(atom)?;
        variant = variant.div(&rhs, Some(atom.mark.clone()))?;
    }

    Ok(Signal::COMPLETE(variant))
}
