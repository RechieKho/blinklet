use super::standard::add::add;
use super::standard::break_fn::break_fn;
use super::standard::closure_fn::closure_fn;
use super::standard::continue_fn::continue_fn;
use super::standard::div::div;
use super::standard::list_fn::list_fn;
use super::standard::mul::mul;
use super::standard::print::print;
use super::standard::rep::rep;
use super::standard::return_fn::return_fn;
use super::standard::scope_fn::scope_fn;
use super::standard::set::set;
use super::standard::sub::sub;
use super::standard::var::var;

use super::signal::Signal;
use super::variant::boolean::Boolean;
use super::variant::command::Command;
use super::variant::list::List;
use super::variant::null::Null;
use super::variant::scope::Scope;
use super::variant::strand::Strand;
use super::variant::table::Table;
use super::variant::Variant;
use crate::backtrace::Backtrace;
use crate::mutex_lock_unwrap;
use crate::parser::atom::generate_commands;
use crate::parser::atom::Atom;
use crate::parser::atom::AtomValue;
use crate::parser::token::tokenize;
use crate::raise_error;
use crate::signal_no_loop_control;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

macro_rules! standard_register_function {
    ($standard:expr, $function:expr) => {{
        $standard.insert(
            String::from(stringify!($function)),
            Variant::COMMAND(Command::wrap_arc($function)),
        );
    }};

    ($standard:expr, $string:expr, $function:expr) => {{
        $standard.insert(
            String::from($string),
            Variant::COMMAND(Command::wrap_arc($function)),
        );
    }};
}

pub type CodeRequestHandler = fn(name: &String) -> Result<String, Backtrace>;

fn default_code_request_handler(name: &String) -> Result<String, Backtrace> {
    let result = fs::read_to_string(name);
    if result.is_err() {
        raise_error!(None, "Unable to fetch code '{}'.", name);
    } else {
        Ok(result.unwrap())
    }
}

/// The runtime that runs Minky code.
pub struct Context {
    standard: Scope,
    pub scopes: Vec<Arc<Mutex<dyn Table>>>,
    pub slots: Vec<Variant>,
    pub code_request_handler: CodeRequestHandler,
}

impl Default for Context {
    fn default() -> Self {
        let mut standard = Scope::default();

        standard_register_function!(standard, var);
        standard_register_function!(standard, set);
        standard_register_function!(standard, print);
        standard_register_function!(standard, rep);
        standard_register_function!(standard, add);
        standard_register_function!(standard, sub);
        standard_register_function!(standard, mul);
        standard_register_function!(standard, div);
        standard_register_function!(standard, "list", list_fn);
        standard_register_function!(standard, "closure", closure_fn);
        standard_register_function!(standard, "scope", scope_fn);
        standard_register_function!(standard, "return", return_fn);
        standard_register_function!(standard, "break", break_fn);
        standard_register_function!(standard, "continue", continue_fn);

        Context {
            standard,
            scopes: Vec::new(),
            slots: Vec::new(),
            code_request_handler: default_code_request_handler,
        }
    }
}

impl Context {
    pub fn resolve_value(&mut self, atom: &Atom) -> Result<Variant, Backtrace> {
        match atom.value {
            AtomValue::COMMAND(ref command) => {
                let signal = self.run_command(command.as_slice())?;
                match signal {
                    Signal::RETURN(value, _) | Signal::COMPLETE(value) => Ok(value),
                    _ => {
                        raise_error!(Some(atom.mark.clone()), "Unexpected control command.");
                    }
                }
            }
            AtomValue::BOOL(boolean) => Ok(Variant::BOOL(Boolean::from(boolean))),
            AtomValue::NULL => Ok(Variant::NULL(Null())),
            AtomValue::STRING(ref string) => {
                let formatted = string.replace("\\n", "\n").replace("\\\\", "\\");
                Ok(Variant::STRAND(Strand::from(formatted)))
            }
            AtomValue::NUMBER(number) => Ok(Variant::NUMBER(number)),
            AtomValue::IDENTIFIER(ref identifier) => {
                // Query standard.
                let value = self.standard.get(identifier);
                if value.is_some() {
                    return Ok(value.unwrap().clone());
                }

                // Query scope.
                let scopes_count = self.scopes.len();
                if scopes_count == 0 {
                    raise_error!(
                        Some(atom.mark.clone()),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                }

                for i in (0..scopes_count).rev() {
                    let table = self.scopes.get(i);
                    if table.is_none() {
                        continue;
                    }
                    let table = table.unwrap();
                    let table = mutex_lock_unwrap!(table, Some(atom.mark.clone()));

                    let value = table.get(identifier);
                    if value.is_none() {
                        continue;
                    }
                    let value = value.unwrap();
                    return Ok(value.clone());
                }

                raise_error!(
                    Some(atom.mark.clone()),
                    "Identifier '{}' is not defined.",
                    identifier
                );
            }
        }
    }

    pub fn resolve_boolean(&mut self, atom: &Atom) -> Result<Boolean, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Variant::BOOL(boolean) = value {
            Ok(boolean)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a boolean.");
        }
    }

    pub fn resolve_number(&mut self, atom: &Atom) -> Result<f64, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Variant::NUMBER(number) = value {
            Ok(number)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a number.");
        }
    }

    pub fn resolve_strand(&mut self, atom: &Atom) -> Result<Strand, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Variant::STRAND(strand) = value {
            Ok(strand)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a string.");
        }
    }

    pub fn resolve_list(&mut self, atom: &Atom) -> Result<List, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Variant::LIST(list) = value {
            Ok(list)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a list.");
        }
    }

    pub fn run_command(&mut self, command: &[Atom]) -> Result<Signal, Backtrace> {
        if command.is_empty() {
            return Ok(Signal::COMPLETE(Variant::NULL(Null())));
        }
        if self.scopes.len() == 0 {
            self.scopes.push(Scope::wrap_arc_mutex())
        }
        let head = command.first().unwrap();

        let value = self.resolve_value(head)?;
        match value {
            Variant::COMMAND(function) => {
                let result = function.call(self, command);
                return Backtrace::trace(result, &head.mark);
            }

            Variant::CLOSURE(closure) => {
                let mut guard = mutex_lock_unwrap!(closure, Some(head.mark.clone()));
                let result = guard.call_mut(self, command);
                return Backtrace::trace(result, &head.mark);
            }

            Variant::TABLE(table) => {
                let signal = Backtrace::trace(self.run_commands(&command[1..], table), &head.mark)?;
                signal_no_loop_control!(signal);
                return Ok(signal);
            }

            _ => {
                raise_error!(
                    Some(head.mark.clone()),
                    "Unexpected value as the head of a command."
                );
            }
        }
    }

    pub fn run_commands(
        &mut self,
        commands: &[Atom],
        scope: Arc<Mutex<dyn Table>>,
    ) -> Result<Signal, Backtrace> {
        if commands.len() == 0 {
            return Ok(Signal::COMPLETE(Variant::TABLE(scope)));
        }

        self.scopes.push(scope);
        for atom in commands.iter() {
            if let AtomValue::COMMAND(ref command) = atom.value {
                let result = self.run_command(command.as_slice());
                if result.is_err() {
                    self.scopes.pop();
                    return result;
                }

                let signal = result.unwrap();
                match signal {
                    Signal::COMPLETE(_) => {}
                    Signal::BREAK(_) | Signal::CONTINUE(_) | Signal::RETURN(_, _) => {
                        self.scopes.pop();
                        return Ok(signal);
                    }
                }
            } else {
                self.scopes.pop();
                raise_error!(Some(atom.mark.clone()), "Expecting command.");
            }
        }
        let scope = self.scopes.pop().unwrap();
        Ok(Signal::COMPLETE(Variant::TABLE(scope)))
    }

    pub fn run_code(&mut self, name: String) -> Result<Variant, Backtrace> {
        let code = (self.code_request_handler)(&name)?;
        let result = tokenize(name, code)?;
        let result = generate_commands(result)?;
        let signal = self.run_commands(result.as_slice(), Scope::wrap_arc_mutex())?;
        match signal {
            Signal::BREAK(ref mark) | Signal::CONTINUE(ref mark) => {
                raise_error!(Some(mark.clone()), "Unexpected control flow structure.");
            }
            Signal::COMPLETE(value) | Signal::RETURN(value, _) => Ok(value),
        }
    }
}
