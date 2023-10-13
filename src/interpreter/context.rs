use super::standard::add_fn::add_fn;
use super::standard::break_fn::break_fn;
use super::standard::closure_fn::closure_fn;
use super::standard::continue_fn::continue_fn;
use super::standard::div_fn::div_fn;
use super::standard::list_fn::list_fn;
use super::standard::list_pop_fn::list_pop_fn;
use super::standard::list_push_fn::list_push_fn;
use super::standard::mul_fn::mul_fn;
use super::standard::print_fn::print_fn;
use super::standard::println_fn::println_fn;
use super::standard::return_fn::return_fn;
use super::standard::set_fn::set_fn;
use super::standard::sub_fn::sub_fn;
use super::standard::table_fn::table_fn;
use super::standard::var_fn::var_fn;
use super::standard::when_fn::when_fn;
use super::standard::while_fn::while_fn;

use super::signal::Signal;
use super::variant::boolean::Boolean;
use super::variant::command::Command;
use super::variant::float::Float;
use super::variant::list::List;
use super::variant::null::Null;
use super::variant::represent::Represent;
use super::variant::strand::Strand;
use super::variant::table::Table;
use super::variant::Variant;
use crate::backtrace::Backtrace;
use crate::log::Log;
use crate::parser::atom::generate_commands;
use crate::parser::atom::Atom;
use crate::parser::atom::AtomValue;
use crate::parser::token::tokenize;
use crate::raise_error;
use hashbrown::HashMap;
use std::fs;

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
    standard: HashMap<&'static str, Variant>,
    pub scopes: Vec<Table>,
    pub slots: Vec<Variant>,
    pub code_request_handler: Box<dyn Fn(&String) -> Result<String, Backtrace> + 'static>,
}

impl Default for Context {
    fn default() -> Self {
        let standard: HashMap<&'static str, Variant> = HashMap::from([
            ("var", Variant::COMMAND(Command::new(var_fn))),
            ("set", Variant::COMMAND(Command::new(set_fn))),
            ("add", Variant::COMMAND(Command::new(add_fn))),
            ("sub", Variant::COMMAND(Command::new(sub_fn))),
            ("mul", Variant::COMMAND(Command::new(mul_fn))),
            ("div", Variant::COMMAND(Command::new(div_fn))),
            ("print", Variant::COMMAND(Command::new(print_fn))),
            ("println", Variant::COMMAND(Command::new(println_fn))),
            ("when", Variant::COMMAND(Command::new(when_fn))),
            ("while", Variant::COMMAND(Command::new(while_fn))),
            ("list", Variant::COMMAND(Command::new(list_fn))),
            ("list_push", Variant::COMMAND(Command::new(list_push_fn))),
            ("list_pop", Variant::COMMAND(Command::new(list_pop_fn))),
            ("closure", Variant::COMMAND(Command::new(closure_fn))),
            ("table", Variant::COMMAND(Command::new(table_fn))),
            ("return", Variant::COMMAND(Command::new(return_fn))),
            ("break", Variant::COMMAND(Command::new(break_fn))),
            ("continue", Variant::COMMAND(Command::new(continue_fn))),
        ]);

        Context {
            standard,
            scopes: Vec::new(),
            slots: Vec::new(),
            code_request_handler: Box::new(default_code_request_handler),
        }
    }
}

impl Context {
    pub fn resolve_variant(&mut self, atom: &Atom) -> Result<Variant, Backtrace> {
        match atom.value {
            AtomValue::STATEMENT(ref command) => {
                let signal = self.run_statement(command.as_slice())?;
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
                let mut result = String::new();
                let replaced = string.replace("\\n", "\n").replace("\\\\", "\\");
                let splitted: Vec<&str> = replaced.split('`').collect();
                if splitted.len() % 2 == 0 {
                    raise_error!(Some(atom.mark.clone()), "Unterminated '`' in string.");
                }
                for (i, slice) in splitted.iter().enumerate() {
                    if i % 2 == 0 {
                        // Even index; Outside the pair of '`'.
                        result.push_str(slice);
                    } else {
                        // Odd index; Between a pair of '`'.
                        if slice.len() == 0 {
                            result.push_str("``");
                        } else {
                            let variant = self.resolve_variant(&Atom::new_identifier(
                                slice.trim().to_string(),
                                atom.mark.clone(),
                            ))?;
                            result.push_str(variant.represent()?.as_str());
                        }
                    }
                }
                Ok(Variant::STRAND(Strand::from(result)))
            }
            AtomValue::FLOAT(float) => Ok(Variant::FLOAT(Float::from(float))),
            AtomValue::IDENTIFIER(ref identifier) => {
                // Query standard.
                let value = self.standard.get(identifier.as_str());
                if value.is_some() {
                    return Ok(value.unwrap().clone());
                }

                // Query table.
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
                    let value = table.get(identifier, Some(atom.mark.clone()))?;
                    if value.is_none() {
                        continue;
                    }
                    return Ok(value.unwrap());
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
        let value = self.resolve_variant(atom)?;
        if let Variant::BOOL(boolean) = value {
            Ok(boolean)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a boolean.");
        }
    }

    pub fn resolve_list(&mut self, atom: &Atom) -> Result<List, Backtrace> {
        let value = self.resolve_variant(atom)?;
        if let Variant::LIST(list) = value {
            Ok(list)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a list.");
        }
    }

    pub fn run_statement(&mut self, statement: &[Atom]) -> Result<Signal, Backtrace> {
        if statement.is_empty() {
            return Ok(Signal::COMPLETE(Variant::NULL(Null())));
        }
        if self.scopes.len() == 0 {
            self.scopes.push(Table::default())
        }
        let head = statement.first().unwrap();
        let body = &statement[1..];

        let value = self.resolve_variant(head)?;
        match value {
            Variant::COMMAND(command) => {
                let result = command.call(self, head, body);
                if result.is_ok() {
                    return result;
                }

                let mut backtrace = result.unwrap_err();
                backtrace.push(Log::trace(head.mark.clone()));
                Err(backtrace)
            }

            Variant::CLOSURE(mut closure) => {
                let result = closure.call_mut(self, body);
                if result.is_ok() {
                    return result;
                }

                let mut backtrace = result.unwrap_err();
                backtrace.push(Log::trace(head.mark.clone()));
                Err(backtrace)
            }

            Variant::TABLE(table) => {
                let result = self.run_statements(&statement[1..], table);
                if result.is_ok() {
                    return result;
                }

                let mut backtrace = result.unwrap_err();
                backtrace.push(Log::trace(head.mark.clone()));
                Err(backtrace)
            }

            _ => {
                raise_error!(
                    Some(head.mark.clone()),
                    "Unexpected value as the head of a command."
                );
            }
        }
    }

    pub fn run_statements(
        &mut self,
        statements: &[Atom],
        table: Table,
    ) -> Result<Signal, Backtrace> {
        if statements.len() == 0 {
            return Ok(Signal::COMPLETE(Variant::TABLE(table)));
        }

        self.scopes.push(table);
        for atom in statements.iter() {
            if let AtomValue::STATEMENT(ref statement) = atom.value {
                let result = self.run_statement(&statement.as_slice());
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
        let table = self.scopes.pop().unwrap();
        Ok(Signal::COMPLETE(Variant::TABLE(table)))
    }

    pub fn run_code(&mut self, name: String) -> Result<Signal, Backtrace> {
        let code = (self.code_request_handler)(&name)?;
        let result = tokenize(name, code)?;
        let result = generate_commands(result)?;
        self.run_statements(result.as_slice(), Table::default())
    }
}
