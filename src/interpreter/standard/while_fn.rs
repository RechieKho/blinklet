use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::table::Table;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count_min, atom_as_identifier};

pub fn while_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 3);
    let identifier = atom_as_identifier!(&body[1]);

    loop {
        let variant = context.resolve_variant(&body[2])?;

        if let Variant::BOOL(boolean) = variant {
            if !boolean.is_true() {
                break;
            }
        } else if let Variant::NULL(_) = variant {
            break;
        }

        let mut table = Table::default();
        table.insert(identifier.clone(), variant, Some(body[2].mark.clone()))?;
        let signal = context.run_statements(&body[3..], table)?;

        match signal {
            Signal::BREAK(_) => break,
            Signal::CONTINUE(_) => continue,
            Signal::RETURN(_, _) => return Ok(signal),
            Signal::COMPLETE(_) => (),
        }
    }

    Ok(Signal::COMPLETE(Variant::NULL(Null::new())))
}
