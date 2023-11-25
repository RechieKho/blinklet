use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::represent::Represent;
use crate::interpreter::variant::strand::Strand;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;
use crate::raise_error;
use crate::{atom_as_identifier, atom_as_statement};
use std::process::Command;

pub fn console_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    let mut output_string = String::new();

    for atom in body {
        let statement = atom_as_statement!(atom);
        let console_head = atom_as_identifier!(&statement[0]);
        let console_args = &statement[1..];

        let mut console_statement = Command::new(console_head);
        for arg in console_args {
            console_statement.arg(format!(
                "\"{}\"",
                context
                    .resolve_variant(arg)?
                    .represent(Some(arg.mark.clone()))?,
            ));
        }

        let output = console_statement.output();

        if output.is_err() {
            raise_error!(
                Some(atom.mark.clone()),
                "Unable to execute the console command '{}'",
                console_head
            );
        }

        let output = output.unwrap();

        if !output.status.success() {
            let output_string = match std::str::from_utf8(&output.stderr) {
                Ok(str) => String::from(str),
                Err(_) => String::new(),
            };
            raise_error!(
                Some(atom.mark.clone()),
                "The console command '{}' fails: {}",
                console_head,
                output_string.trim().trim_matches('"')
            );
        }

        output_string.push('\n');
        output_string += match std::str::from_utf8(&output.stdout) {
            Ok(str) => String::from(str),
            Err(_) => {
                raise_error!(
                    Some(atom.mark.clone()),
                    "The console command '{}' output cannot be read into string.",
                    console_head
                );
            }
        }
        .trim()
        .trim_matches('"');
    }

    Ok(Signal::COMPLETE(Variant::STRAND(Strand::from(
        output_string.trim(),
    ))))
}
