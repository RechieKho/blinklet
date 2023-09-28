use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::backtrace::Backtrace;
use crate::mutex_force_lock;
use crate::parser::command::generate_commands;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::parser::lexer::lex;
use crate::raise_error;
use crate::signal_no_loop_control;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

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
    pub scopes: Vec<Arc<Mutex<Object>>>,
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
    pub fn new(scopes: Vec<Arc<Mutex<Object>>>, slots: Vec<Value>) -> Context {
        Context {
            scopes,
            slots,
            code_request_handler: default_code_request_handler,
        }
    }

    pub fn resolve_value(&mut self, atom: &Atom) -> Result<Value, Backtrace> {
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
            AtomValue::BOOL(boolean) => Ok(Value::BOOL(boolean)),
            AtomValue::NULL => Ok(Value::NULL),
            AtomValue::STRING(ref string) => Ok(Value::STRING(string.clone())),
            AtomValue::NUMBER(number) => Ok(Value::NUMBER(number)),
            AtomValue::IDENTIFIER(ref identifier) => {
                let scopes_count = self.scopes.len();
                if scopes_count == 0 {
                    raise_error!(
                        Some(atom.mark.clone()),
                        "Identifier '{}' is not defined.",
                        identifier
                    );
                }

                for i in (0..scopes_count).rev() {
                    let object = self.scopes.get_mut(i);
                    if object.is_none() {
                        continue;
                    }
                    let object = object.unwrap();
                    let mut object = mutex_force_lock!(object, atom.mark.clone());

                    let value = object.content.get_mut(identifier);
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

    pub fn run_command(&mut self, command: &[Atom]) -> Result<Signal, Backtrace> {
        if command.is_empty() {
            return Ok(Signal::COMPLETE(Value::NULL));
        }
        if self.scopes.len() == 0 {
            self.scopes.push(Object::with_mutex())
        }
        let head = command.first().unwrap();

        let value = self.resolve_value(head)?;
        match value {
            Value::FUNCTION(function) => {
                let result = function(self, command);
                return Backtrace::trace(result, &head.mark);
            }

            Value::CLOSURE(closure) => {
                let mut guard = mutex_force_lock!(closure, head.mark.clone());
                let result = guard.call_mut(self, command);
                return Backtrace::trace(result, &head.mark);
            }

            Value::OBJECT(object) => {
                let signal =
                    Backtrace::trace(self.run_commands(&command[1..], object), &head.mark)?;
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
        scope: Arc<Mutex<Object>>,
    ) -> Result<Signal, Backtrace> {
        if commands.len() == 0 {
            return Ok(Signal::COMPLETE(Value::OBJECT(scope)));
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
        Ok(Signal::COMPLETE(Value::OBJECT(scope)))
    }

    pub fn run_code(&mut self, name: String) -> Result<Value, Backtrace> {
        let code = (self.code_request_handler)(&name)?;
        let result = lex(name, code)?;
        let result = generate_commands(result)?;
        let signal = self.run_commands(result.as_slice(), Object::with_mutex())?;
        match signal {
            Signal::BREAK(ref mark) | Signal::CONTINUE(ref mark) => {
                raise_error!(Some(mark.clone()), "Unexpected control flow structure.");
            }
            Signal::COMPLETE(value) | Signal::RETURN(value, _) => Ok(value),
        }
    }
}
