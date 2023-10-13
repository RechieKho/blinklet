use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::table::Table;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count_min, atom_as_identifier};

pub fn while_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, Some(head.mark.clone()), 2);
    let identifier = atom_as_identifier!(&body[0]);
    loop {
        let variant = context.resolve_variant(&body[1])?;
        match variant {
            Variant::BOOL(boolean) => {
                if !boolean.is_true() {
                    break;
                }
            }
            Variant::NULL(_) => break,
            _ => (),
        }
        let mut table = Table::default();
        table.insert(identifier.clone(), variant, Some(body[1].mark.clone()))?;
        let signal = context.run_statements(&body[2..], table)?;
        match signal {
            Signal::BREAK(_) => break,
            Signal::CONTINUE(_) => continue,
            Signal::RETURN(_, _) => return Ok(signal),
            Signal::COMPLETE(_) => (),
        }
    }
    Ok(Signal::COMPLETE(Variant::NULL(Null::new())))
}
