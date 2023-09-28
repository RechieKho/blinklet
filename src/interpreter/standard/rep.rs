use crate::assert_atoms_count_min;
use crate::atom_as_identifier;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::object::Object;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
use crate::mutex_lock_unwrap;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::raise_error;

pub fn rep(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 5);

    let first_atom = body.first().unwrap();

    let commands = &body[5..];
    if commands.len() == 0 {
        return Ok(Signal::COMPLETE(Value::NULL));
    }

    let step = context.resolve_number(&body[4])?;
    if step.is_sign_negative() {
        raise_error!(
            Some(first_atom.mark.clone()),
            "Step of repetition must be positive, system will increment/decrement to approach end value for you."
        );
    }

    let index_identifier = atom_as_identifier!(&body[1]);
    let start = context.resolve_number(&body[2])?;
    let end = context.resolve_number(&body[3])?;

    let mut index = start;

    loop {
        let scope = Object::with_mutex();
        {
            let mut scope = mutex_lock_unwrap!(scope, first_atom.mark.clone());
            scope
                .content
                .insert(index_identifier.clone(), Value::NUMBER(index));
        }

        let signal = context.run_commands(commands, scope)?;
        match signal {
            Signal::RETURN(_, _) => {
                return Ok(signal);
            }
            Signal::BREAK(_) => {
                break;
            }
            _ => {}
        }

        if start < end {
            index += step;
            if index >= end {
                break;
            }
        } else if start > end {
            index -= step;
            if index <= end {
                break;
            }
        } else {
            break;
        }
    }

    Ok(Signal::COMPLETE(Value::NULL))
}
