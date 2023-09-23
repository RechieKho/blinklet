use super::backtrace::Backtrace;
use super::function::Function;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::log::Log;
use crate::mark::Mark;
use crate::parser::command::generate_commands;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::parser::lexer::lex;
use crate::raise_backtrace_error;
use std::fs;
use std::rc::Rc;

pub type CodeRequestHandler = fn(name: &String) -> Result<String, Log>;

fn default_code_request_handler(name: &String) -> Result<String, Log> {
    let result = fs::read_to_string(name);
    if result.is_err() {
        Err(Log::error(
            format!("Unable to fetch code '{}'.", name),
            None,
        ))
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
                }
            }
            AtomValue::BOOL(boolean) => Ok(Value::BOOL(boolean)),
            AtomValue::NULL => Ok(Value::NULL),
            AtomValue::STRING(ref string) => Ok(Value::STRING(string.clone())),
            AtomValue::NUMBER(number) => Ok(Value::NUMBER(number)),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    Ok(optional_value.unwrap().clone())
                }
            }
        }
    }

    pub fn resolve_bool(&self, atom: &Atom) -> Result<bool, Backtrace> {
        match atom.value {
            AtomValue::BOOL(boolean) => Ok(boolean),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::BOOL(boolean) => Ok(*boolean),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not a boolean",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not a boolean.");
            }
        }
    }

    pub fn resolve_number(&self, atom: &Atom) -> Result<f64, Backtrace> {
        match atom.value {
            AtomValue::NUMBER(number) => Ok(number),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::NUMBER(number) => Ok(*number),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not a number",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not a number.");
            }
        }
    }

    pub fn resolve_string(&self, atom: &Atom) -> Result<String, Backtrace> {
        match atom.value {
            AtomValue::STRING(ref string) => Ok(string.clone()),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::STRING(string) => Ok(string.clone()),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not a string.",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not a string.");
            }
        }
    }

    pub fn resolve_list(&self, atom: &Atom) -> Result<Vec<Value>, Backtrace> {
        match atom.value {
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::LIST(list) => Ok(list.clone()),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not a list.",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not a list.");
            }
        }
    }

    pub fn resolve_object(&self, atom: &Atom) -> Result<Object, Backtrace> {
        match atom.value {
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::OBJECT(object) => Ok(object.clone()),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not an object.",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not an object.");
            }
        }
    }

    pub fn resolve_function(&self, atom: &Atom) -> Result<Rc<dyn Function>, Backtrace> {
        match atom.value {
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    raise_backtrace_error!(
                        atom.mark.clone(),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                } else {
                    match optional_value.unwrap() {
                        Value::FUNCTION(function) => Ok(function.clone()),
                        _ => {
                            raise_backtrace_error!(
                                atom.mark.clone(),
                                "'{}' is not a function.",
                                identifier
                            );
                        }
                    }
                }
            }
            _ => {
                raise_backtrace_error!(atom.mark.clone(), "Value given is not a function.");
            }
        }
    }

    pub fn run_command(&mut self, command: &[Atom]) -> Result<Signal, Backtrace> {
        if command.is_empty() {
            return Ok(Signal::COMPLETE(Value::NULL));
        }
        if self.scopes.len() == 0 {
            self.scopes.push(Object::default())
        }
        let head = command.first().unwrap();
        let function = self.resolve_function(head);
        if function.is_ok() {
            let function = function.unwrap();
            let result = function.call(self, command);
            return result;
        }

        let object = self.resolve_object(head);
        if object.is_ok() {
            let object = object.unwrap();
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

        Err(Backtrace::new(Log::error(
            format!("Unexpected value as the head of a command."),
            head.mark.clone(),
        )))
    }

    pub fn run_code(&mut self, name: String) -> Result<Value, Backtrace> {
        let code = (self.code_request_handler)(&name);
        if code.is_err() {
            return Err(Backtrace::new(code.unwrap_err()));
        }
        let code = code.unwrap();

        let result = lex(name, code);
        if result.is_err() {
            return Err(Backtrace::new(result.unwrap_err()));
        }
        let result = result.unwrap();

        let result = generate_commands(result);
        if result.is_err() {
            return Err(Backtrace::new(result.unwrap_err()));
        }
        let mut result = result.unwrap();

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
