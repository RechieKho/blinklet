use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::variant_ops::VariantMul;
use crate::parser::atom::Atom;

pub fn mul_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, Some(head.mark.clone()), 2);
    let mut variant = context.resolve_variant(&body[1])?;
    for atom in body.iter().skip(1) {
        let rhs = context.resolve_variant(atom)?;
        variant = variant.mul(&rhs, Some(atom.mark.clone()))?;
    }
    Ok(Signal::COMPLETE(variant))
}
