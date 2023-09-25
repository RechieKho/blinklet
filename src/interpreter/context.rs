use super::function::Function;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::backtrace::Backtrace;
use crate::log::Log;
use crate::parser::command::generate_commands;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::parser::lexer::lex;
use crate::raise_error;
use std::fs;
use std::rc::Rc;

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
    pub scopes: Vec<Object>,
    pub slots: Vec<Value>,
    pub code_request_handler: CodeRequestHandler,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            scopes: Vec::new(),
            slots: Vec::new(),
            code_request_handler: default_code_request_handler,
        }
    }
}

impl Context {
    pub fn new(scopes: Vec<Object>, slots: Vec<Value>) -> Context {
        Context {
            scopes,
            slots,
            code_request_handler: default_code_request_handler,
        }
    }

    fn get_value<'context>(&'context self, identifier: &str) -> Option<&'context Value> {
        let scopes_count = self.scopes.len();
        if scopes_count == 0 {
            return None;
        }
        for i in (0..scopes_count).rev() {
            let object = self.scopes.get(i);
            if object.is_none() {
                continue;
            }
            let object = object.unwrap();

            let value = object.content.get(identifier);
            if value.is_none() {
                continue;
            }
            let value = value.unwrap();

            return Some(value);
        }
        None
    }

    pub fn resolve_value(&mut self, atom: &Atom) -> Result<Value, Backtrace> {
        match atom.value {
            AtomValue::COMMAND(ref command) => {
                let signal = self.run_command(command.as_slice())?;
                match signal {
                    Signal::RETURN(value) => Ok(value),
                    Signal::COMPLETE(value) => Ok(value),
                    _ => {
                        raise_error!(Some(atom.mark.clone()), "Unexpected control command.");
                    }
                }
            }
            AtomValue::BOOL(boolean) => Ok(Value::BOOL(boolean)),
            AtomValue::NULL => Ok(Value::NULL),
            AtomValue::STRING(ref string) => Ok(Value::STRING(string.clone())),
            AtomValue::NUMBER(number) => Ok(Value::NUMBER(number)),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_error!(
                        Some(atom.mark.clone()),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    Ok(optional_value.unwrap().clone())
                }
            }
        }
    }

    pub fn resolve_bool(&mut self, atom: &Atom) -> Result<bool, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::BOOL(boolean) = value {
            Ok(boolean)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not a boolean.");
        }
    }

    pub fn resolve_number(&mut self, atom: &Atom) -> Result<f64, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::NUMBER(number) = value {
            Ok(number)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not a number.");
        }
    }

    pub fn resolve_string(&mut self, atom: &Atom) -> Result<String, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::STRING(string) = value {
            Ok(string)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not a string.");
        }
    }

    pub fn resolve_list(&mut self, atom: &Atom) -> Result<Vec<Value>, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::LIST(list) = value {
            Ok(list)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not a list.");
        }
    }

    pub fn resolve_object(&mut self, atom: &Atom) -> Result<Object, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::OBJECT(object) = value {
            Ok(object)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not an object.");
        }
    }

    pub fn resolve_function(&mut self, atom: &Atom) -> Result<Rc<dyn Function>, Backtrace> {
        let value = self.resolve_value(atom)?;
        if let Value::FUNCTION(function) = value {
            Ok(function)
        } else {
            raise_error!(Some(atom.mark.clone()), "Value given is not an object.");
        }
    }

    pub fn run_command(&mut self, command: &[Atom]) -> Result<Signal, Backtrace> {
        if self.scopes.len() == 0 {
            self.scopes.push(Object::default())
        }
        if command.is_empty() {
            return Ok(Signal::COMPLETE(Value::NULL));
        }
        let head = command.first().unwrap();

        let value = self.resolve_value(head)?;
        match value {
            Value::FUNCTION(function) => {
                let result = function.call(self, command);
                return result;
            }

            Value::OBJECT(object) => {
                self.scopes.push(object);
                let mut final_result: Result<Signal, Backtrace> = Ok(Signal::COMPLETE(Value::NULL));
                for atom in command.iter().skip(1) {
                    if let AtomValue::COMMAND(ref command) = atom.value {
                        let result = self.run_command(command.as_slice());
                        if result.is_err() {
                            final_result = result;
                            break;
                        }
                        let signal = result.unwrap();
                        if let Signal::RETURN(_) = signal {
                            final_result = Ok(signal);
                            break;
                        }
                    }
                }
                let object = self.scopes.pop().unwrap();
                let signal = Backtrace::trace(final_result, &head.mark)?;
                if let Signal::RETURN(_) = signal {
                    return Ok(signal);
                } else {
                    return Ok(Signal::COMPLETE(Value::OBJECT(object)));
                }
            }

            _ => {
                raise_error!(
                    Some(head.mark.clone()),
                    "Unexpected value as the head of a command."
                );
            }
        }
    }

    pub fn run_code(&mut self, name: String) -> Result<Value, Backtrace> {
        let code = (self.code_request_handler)(&name)?;
        let result = lex(name, code)?;
        let mut result = generate_commands(result)?;

        if self.scopes.len() == 0 {
            self.scopes.push(Object::default())
        }

        for command in result.drain(..) {
            let signal = self.run_command(command.as_slice())?;
            if let Signal::RETURN(value) = signal {
                return Ok(value);
            }
        }

        let object = self.scopes.pop().unwrap();
        Ok(Value::OBJECT(object))
    }
}
