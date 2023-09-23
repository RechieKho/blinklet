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

    pub fn declare(&mut self, identifier: String, value: Value) -> Option<Value> {
        self.scopes.last_mut()?.content.insert(identifier, value)
    }

    pub fn set(&mut self, identifier: String, value: Value) -> Result<Value, Log> {
        let scopes_count = self.scopes.len();
        if scopes_count == 0 {
            return Err(Log::error(format!("Unable to set value."), None));
        }
        for i in (0..scopes_count).rev() {
            let object = self.scopes.get_mut(i);
            if object.is_none() {
                continue;
            }
            let object = object.unwrap();

            if object.content.contains_key(identifier.as_str()) {
                return Ok(object.content.insert(identifier, value).unwrap());
            }
        }
        Err(Log::error(format!("Unable to set value."), None))
    }

    pub fn get_value<'context>(&'context self, identifier: &str) -> Option<&'context Value> {
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
                    Err(Backtrace::new(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    )))
                } else {
                    Ok(optional_value.unwrap().clone())
                }
            }
        }
    }

    pub fn resolve_bool(&self, atom: &Atom) -> Result<bool, Log> {
        match atom.value {
            AtomValue::BOOL(boolean) => Ok(boolean),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    Err(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    ))
                } else {
                    match optional_value.unwrap() {
                        Value::BOOL(boolean) => Ok(*boolean),
                        _ => Err(Log::error(
                            format!("'{}' is not a boolean.", identifier),
                            atom.mark.clone(),
                        )),
                    }
                }
            }
            _ => Err(Log::error(
                format!("Value given is not a boolean."),
                atom.mark.clone(),
            )),
        }
    }

    pub fn resolve_number(&self, atom: &Atom) -> Result<f64, Log> {
        match atom.value {
            AtomValue::NUMBER(number) => Ok(number),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    Err(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    ))
                } else {
                    match optional_value.unwrap() {
                        Value::NUMBER(number) => Ok(*number),
                        _ => Err(Log::error(
                            format!("'{}' is not a number.", identifier),
                            atom.mark.clone(),
                        )),
                    }
                }
            }
            _ => Err(Log::error(
                format!("Value given is not a number."),
                atom.mark.clone(),
            )),
        }
    }

    pub fn resolve_string(&self, atom: &Atom) -> Result<String, Log> {
        match atom.value {
            AtomValue::STRING(ref string) => Ok(string.clone()),
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    Err(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    ))
                } else {
                    match optional_value.unwrap() {
                        Value::STRING(string) => Ok(string.clone()),
                        _ => Err(Log::error(
                            format!("'{}' is not a string.", identifier),
                            atom.mark.clone(),
                        )),
                    }
                }
            }
            _ => Err(Log::error(
                format!("Value given is not a string."),
                atom.mark.clone(),
            )),
        }
    }

    pub fn resolve_list(&self, atom: &Atom) -> Result<Vec<Value>, Log> {
        match atom.value {
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    Err(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    ))
                } else {
                    match optional_value.unwrap() {
                        Value::LIST(list) => Ok(list.clone()),
                        _ => Err(Log::error(
                            format!("'{}' is not a list.", identifier),
                            atom.mark.clone(),
                        )),
                    }
                }
            }
            _ => Err(Log::error(
                format!("Value given is not a list."),
                atom.mark.clone(),
            )),
        }
    }

    pub fn resolve_object(&self, atom: &Atom) -> Result<Object, Log> {
        match atom.value {
            AtomValue::IDENTIFIER(ref identifier) => {
                let optional_value = self.get_value(identifier.as_str());
                if optional_value.is_none() {
                    Err(Log::error(
                        format!("Identifier '{}' is not defined.", identifier),
                        atom.mark.clone(),
                    ))
                } else {
                    match optional_value.unwrap() {
                        Value::OBJECT(object) => Ok(object.clone()),
                        _ => Err(Log::error(
                            format!("'{}' is not an object.", identifier),
                            atom.mark.clone(),
                        )),
                    }
                }
            }
            _ => Err(Log::error(
                format!("Value given is not an object."),
                atom.mark.clone(),
            )),
        }
    }

    pub fn resolve_function(&self, atom: &Atom) -> Result<Rc<dyn Function>, Log> {
        if let AtomValue::IDENTIFIER(ref identifier) = atom.value {
            let optional_value = self.get_value(identifier.as_str());
            if optional_value.is_none() {
                Err(Log::error(
                    format!("Identifier '{}' is not defined.", identifier),
                    atom.mark.clone(),
                ))
            } else if let Value::FUNCTION(function) = optional_value.unwrap() {
                Ok(function.clone())
            } else {
                Err(Log::error(
                    format!("'{}' is not a function.", identifier),
                    atom.mark.clone(),
                ))
            }
        } else {
            Err(Log::error(
                format!("Value given is not a function."),
                atom.mark.clone(),
            ))
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
            let mut final_result : Result<Signal, Backtrace> = Ok(Signal::COMPLETE(Value::NULL));
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
