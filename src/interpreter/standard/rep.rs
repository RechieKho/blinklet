use crate::assert_atoms_count_min;
use crate::atom_as_identifier;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::object::Object;
use crate::interpreter::signal::Signal;
use crate::interpreter::value::Value;
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
        let mut scope = Object::default();
        scope
            .content
            .insert(index_identifier.clone(), Value::NUMBER(index));
        context.scopes.push(scope);
        let mut final_result: Result<Signal, Backtrace> = Ok(Signal::COMPLETE(Value::NULL));
        for atom in commands.iter() {
            if let AtomValue::COMMAND(ref command) = atom.value {
                let result = context.run_command(command.as_slice());
                if result.is_err() {
                    final_result = result;
                    break;
                }
                let signal = result.unwrap();
                match signal {
                    Signal::BREAK | Signal::CONTINUE | Signal::RETURN(_) => {
                        final_result = Ok(signal);
                        break;
                    }
                    _ => {}
                }
            }
        }
        context.scopes.pop();
        let signal = final_result?;
        match signal {
            Signal::RETURN(_) => {
                return Ok(signal);
            }
            Signal::BREAK => {
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
