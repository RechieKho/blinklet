use super::function::Function;
use super::object::Object;
use super::signal::Signal;
use super::value::Value;
use crate::error::Error;
use crate::parser::command::generate_commands;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use crate::parser::lexer::lex;
use std::rc::Rc;

/// The runtime that runs Minky code.
pub struct Context<'code> {
    pub scopes: Vec<Object<'code>>,
    pub slots: Vec<Value<'code>>,
}

impl<'code> Default for Context<'code> {
    fn default() -> Self {
        Context {
            scopes: vec![Object::default()],
            slots: Vec::new(),
        }
    }
}

impl<'code> Context<'code> {
    pub fn declare(&mut self, identifier: String, value: Value<'code>) -> Option<Value<'code>> {
        self.scopes.last_mut()?.content.insert(identifier, value)
    }

    pub fn set(
        &mut self,
        identifier: String,
        value: Value<'code>,
    ) -> Result<Value<'code>, Error<'code>> {
        let scopes_count = self.scopes.len();
        if scopes_count == 0 {
            return Err(Error {
                message: format!("Unable to set value."),
                mark: None,
            });
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
        Err(Error {
            message: format!("Unable to set value."),
            mark: None,
        })
    }

    pub fn get_value<'context>(&'context self, identifier: &str) -> Option<&'context Value<'code>> {
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

    pub fn resolve_value(&mut self, atom: &Atom<'code>) -> Result<Value<'code>, Error<'code>> {
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
            AtomValue::STRING(string) => Ok(Value::STRING(String::from(string))),
            AtomValue::NUMBER(number) => Ok(Value::NUMBER(number)),
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    Ok(optional_value.unwrap().clone())
                }
            }
        }
    }

    pub fn resolve_bool(&self, atom: &Atom<'code>) -> Result<bool, Error<'code>> {
        match atom.value {
            AtomValue::BOOL(boolean) => Ok(boolean),
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    match optional_value.unwrap() {
                        Value::BOOL(boolean) => Ok(*boolean),
                        _ => Err(Error {
                            message: format!("'{}' is not a boolean.", identifier),
                            mark: atom.mark.clone(),
                        }),
                    }
                }
            }
            _ => Err(Error {
                message: format!("Value given is not a boolean."),
                mark: atom.mark.clone(),
            }),
        }
    }

    pub fn resolve_number(&self, atom: &Atom<'code>) -> Result<f64, Error<'code>> {
        match atom.value {
            AtomValue::NUMBER(number) => Ok(number),
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    match optional_value.unwrap() {
                        Value::NUMBER(number) => Ok(*number),
                        _ => Err(Error {
                            message: format!("'{}' is not a number.", identifier),
                            mark: atom.mark.clone(),
                        }),
                    }
                }
            }
            _ => Err(Error {
                message: format!("Value given is not a number."),
                mark: atom.mark.clone(),
            }),
        }
    }

    pub fn resolve_string(&self, atom: &Atom<'code>) -> Result<String, Error<'code>> {
        match atom.value {
            AtomValue::STRING(string) => Ok(String::from(string)),
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    match optional_value.unwrap() {
                        Value::STRING(string) => Ok(string.clone()),
                        _ => Err(Error {
                            message: format!("'{}' is not a string.", identifier),
                            mark: atom.mark.clone(),
                        }),
                    }
                }
            }
            _ => Err(Error {
                message: format!("Value given is not a string."),
                mark: atom.mark.clone(),
            }),
        }
    }

    pub fn resolve_list(&self, atom: &Atom<'code>) -> Result<Vec<Value<'code>>, Error<'code>> {
        match atom.value {
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    match optional_value.unwrap() {
                        Value::LIST(list) => Ok(list.clone()),
                        _ => Err(Error {
                            message: format!("'{}' is not a list.", identifier),
                            mark: atom.mark.clone(),
                        }),
                    }
                }
            }
            _ => Err(Error {
                message: format!("Value given is not a list."),
                mark: atom.mark.clone(),
            }),
        }
    }

    pub fn resolve_object(&self, atom: &Atom<'code>) -> Result<Object<'code>, Error<'code>> {
        match atom.value {
            AtomValue::IDENTIFIER(identifier) => {
                let optional_value = self.get_value(identifier);
                if optional_value.is_none() {
                    Err(Error {
                        message: format!("Identifier '{}' is not defined.", identifier),
                        mark: atom.mark.clone(),
                    })
                } else {
                    match optional_value.unwrap() {
                        Value::OBJECT(object) => Ok(object.clone()),
                        _ => Err(Error {
                            message: format!("'{}' is not an object.", identifier),
                            mark: atom.mark.clone(),
                        }),
                    }
                }
            }
            _ => Err(Error {
                message: format!("Value given is not an object."),
                mark: atom.mark.clone(),
            }),
        }
    }

    pub fn resolve_function(
        &self,
        atom: &Atom<'code>,
    ) -> Result<Rc<dyn Function<'code> + 'code>, Error<'code>> {
        if let AtomValue::IDENTIFIER(identifier) = atom.value {
            let optional_value = self.get_value(identifier);
            if optional_value.is_none() {
                Err(Error {
                    message: format!("Identifier '{}' is not defined.", identifier),
                    mark: atom.mark.clone(),
                })
            } else if let Value::FUNCTION(function) = optional_value.unwrap() {
                Ok(function.clone())
            } else {
                Err(Error {
                    message: format!("'{}' is not a function.", identifier),
                    mark: atom.mark.clone(),
                })
            }
        } else {
            Err(Error {
                message: format!("Value given is not a function."),
                mark: atom.mark.clone(),
            })
        }
    }

    pub fn run_command(&mut self, command: &[Atom<'code>]) -> Result<Signal<'code>, Error<'code>> {
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
            self.scopes.push(Object::default());
            let result = function.call(self, &command[1..]);
            self.scopes.pop();
            return result;
        }

        let object = self.resolve_object(head);
        if object.is_ok() {
            let object = object.unwrap();
            self.scopes.push(object);
            let signal = self.run_command(&command[1..]);
            let object = self.scopes.pop().unwrap();
            let signal = signal?;
            if let Signal::RETURN(_) = signal {
                return Ok(signal);
            } else {
                return Ok(Signal::COMPLETE(Value::OBJECT(object)));
            }
        }

        if let AtomValue::IDENTIFIER(identifier) = head.value {
            let value = self.slots.pop().unwrap_or(Value::NULL);
            self.declare(String::from(identifier), value);
            return Ok(Signal::COMPLETE(Value::NULL));
        }

        Err(Error {
            message: format!("Unexpected value as the head of a command."),
            mark: head.mark.clone(),
        })
    }

    pub fn run_code(&mut self, code: &'code String) -> Result<(), Error<'code>> {
        let result = lex(code)?;
        let result = generate_commands(&result)?;
        for command in result.iter() {
            self.run_command(command)?;
        }
        Ok(())
    }
}
