use super::{
    object::Object,
    value::{Register, Value},
};
use crate::parser::{
    command::{generate_commands, Atom},
    lexer::lex,
};

#[derive(Default, Clone)]
pub struct EvaluationContext<'code> {
    scopes: Vec<Object<'code>>,
    slots: Vec<Value<'code>>,
}

impl<'code> EvaluationContext<'code> {
    pub fn get(&self, k: &String) -> Option<&Register<'code>> {
        for scope in self.scopes.iter() {
            let register = scope.content.get(k);
            if register.is_some() {
                return register;
            }
        }
        None
    }

    pub fn run_command(&mut self, command: &[Atom<'code>]) {
        if command.is_empty() {
            return;
        }
        let head = command.first().unwrap();
        let body = &command[1..];
        if self.scopes.is_empty() {
            self.scopes.push(Object::default());
        }
        match head {
            Atom::WORD(d, _) => {
                let word = String::from(*d);
                for scope in self.scopes.clone().iter_mut().rev() {
                    let optional_value = scope.content.remove(&word);
                    if optional_value.is_none() {
                        continue;
                    }
                    match optional_value.unwrap().value {
                        Value::FUNCTION(function) => {
                            self.slots.clear();
                            let value = function.call(self, body);
                            self.slots.push(value);
                            return;
                        }
                        Value::OBJECT(object) => {
                            self.scopes.push(object);
                            self.run_command(body);
                            self.slots.clear();
                            self.slots.push(Value::OBJECT(self.scopes.pop().unwrap()));
                            return;
                        }
                        _ => {
                            panic!("Unexpected value as the head of a command.");
                        }
                    }
                }
                panic!("Undefined identifier as the head of a command. ");
            }
            _ => unreachable!("Non-word as the head of a command should be unreachable."),
        }
    }

    pub fn run_code(&mut self, code: &'code String) {
        let result = lex(code);
        if result.is_err() {
            let error = result.unwrap_err();
            panic!(
                "Lexical analysis error: {} [line {}, column {}]",
                error.message, error.position.0, error.position.1
            );
        }

        let result = generate_commands(&result.unwrap());
        if result.is_err() {
            let error = result.unwrap_err();
            panic!(
                "Command generation error: {} [line {}, column {}]",
                error.message, error.position.0, error.position.1
            );
        }

        for command in result.unwrap().iter() {
            self.run_command(command);
        }
    }
}
