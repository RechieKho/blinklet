use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::boolean::Boolean;
use crate::interpreter::variant::variant_ops::VariantEq;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn eq_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 2);
    let variant = context.resolve_variant(&body[0])?;
    for atom in body.iter().skip(1) {
        let rhs = context.resolve_variant(atom)?;
        match variant.eq(&rhs, Some(atom.mark.clone()))? {
            Variant::BOOL(boolean) => {
                if !boolean.is_true() {
                    return Ok(Signal::COMPLETE(Variant::BOOL(Boolean::from(false))));
                }
            }
            _ => (),
        }
    }
    Ok(Signal::COMPLETE(Variant::BOOL(Boolean::from(true))))
}
