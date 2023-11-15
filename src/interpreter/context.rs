use super::resource::system_resource::SystemResource;
use super::resource::Resource;
use super::resource::ResourcePath;
use super::standard::add_fn::add_fn;
use super::standard::break_fn::break_fn;
use super::standard::closure_fn::closure_fn;
use super::standard::console_fn::console_fn;
use super::standard::continue_fn::continue_fn;
use super::standard::div_fn::div_fn;
use super::standard::eq_fn::eq_fn;
use super::standard::g_fn::g_fn;
use super::standard::ge_fn::ge_fn;
use super::standard::import_fn::import_fn;
use super::standard::l_fn::l_fn;
use super::standard::le_fn::le_fn;
use super::standard::list_fn::list_fn;
use super::standard::list_get_fn::list_get_fn;
use super::standard::list_length_fn::list_length_fn;
use super::standard::list_pop_fn::list_pop_fn;
use super::standard::list_push_fn::list_push_fn;
use super::standard::mul_fn::mul_fn;
use super::standard::parameter_fn::parameter_fn;
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

/// The runtime that runs Minky code.
pub struct Context {
    standard: HashMap<&'static str, Variant>,
    pub(super) scopes: Vec<Table>,
    pub slots: Vec<Variant>,
    pub resource: Box<dyn Resource>,
}

impl Context {
    pub fn new() -> Result<Self, Backtrace> {
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
            ("list-get", Variant::COMMAND(Command::new(list_get_fn))),
            ("list-push", Variant::COMMAND(Command::new(list_push_fn))),
            ("list-pop", Variant::COMMAND(Command::new(list_pop_fn))),
            (
                "list-length",
                Variant::COMMAND(Command::new(list_length_fn)),
            ),
            ("closure", Variant::COMMAND(Command::new(closure_fn))),
            ("parameter", Variant::COMMAND(Command::new(parameter_fn))),
            ("table", Variant::COMMAND(Command::new(table_fn))),
            ("return", Variant::COMMAND(Command::new(return_fn))),
            ("break", Variant::COMMAND(Command::new(break_fn))),
            ("continue", Variant::COMMAND(Command::new(continue_fn))),
            ("import", Variant::COMMAND(Command::new(import_fn))),
            ("=", Variant::COMMAND(Command::new(eq_fn))),
            (">=", Variant::COMMAND(Command::new(ge_fn))),
            (">", Variant::COMMAND(Command::new(g_fn))),
            ("<=", Variant::COMMAND(Command::new(le_fn))),
            ("<", Variant::COMMAND(Command::new(l_fn))),
            ("console", Variant::COMMAND(Command::new(console_fn))),
        ]);

        let mut context = Context {
            standard,
            scopes: Vec::new(),
            slots: Vec::new(),
            resource: Box::new(SystemResource::default()),
        };

        let make_list_iter_fn_code = include_str!("./standard/make_list_iter_fn.k");
        context.install_code("make-list-iter", String::from(make_list_iter_fn_code))?;

        Ok(context)
    }

    pub fn resolve_variant(&mut self, atom: &Atom) -> Result<Variant, Backtrace> {
        match atom.value {
            AtomValue::STATEMENT(ref statement) => {
                let signal = self.run_statement(statement.as_slice())?;
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
                            result.push_str(variant.represent(Some(atom.mark.clone()))?.as_str());
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

    pub fn resolve_float(&mut self, atom: &Atom) -> Result<Float, Backtrace> {
        let value = self.resolve_variant(atom)?;
        if let Variant::FLOAT(float) = value {
            Ok(float)
        } else {
            raise_error!(Some(atom.mark.clone()), "Variant given is not a float.");
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
                raise_error!(Some(atom.mark.clone()), "Expecting statement.");
            }
        }
        let table = self.scopes.pop().unwrap();
        Ok(Signal::COMPLETE(Variant::TABLE(table)))
    }

    pub fn run_resource(&mut self, mut path: ResourcePath) -> Result<Signal, Backtrace> {
        let module_name: String = path.clone().into();
        let previous_prefix = self.resource.get_prefix().clone();
        let mut new_prefix = self.resource.get_prefix().clone();
        let _ = new_prefix.append(&mut path.remove_parent_path());
        self.resource.set_prefix(new_prefix);
        let code = self.resource.get_code(path)?;
        let result = self.run_code(module_name, code);
        self.resource.set_prefix(previous_prefix);
        result
    }

    pub fn run_code(&mut self, name: String, code: String) -> Result<Signal, Backtrace> {
        let result = tokenize(name, code)?;
        let result = generate_commands(result)?;
        self.run_statements(result.as_slice(), Table::default())
    }

    pub fn install_code(&mut self, name: &'static str, code: String) -> Result<(), Backtrace> {
        let signal = self.run_code(String::from(name), code)?;
        match signal {
            Signal::COMPLETE(value) | Signal::RETURN(value, _) => self.standard.insert(name, value),
            _ => None,
        };
        Ok(())
    }
}
