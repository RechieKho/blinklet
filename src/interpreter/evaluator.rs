use super::{object::Object, value::Value};
use crate::parser::{lexer::lex, command::{generate_commands, Atom}};

#[derive(Default, Clone)]
pub struct EvaluationContext<'code> {
    scopes: Vec<Object<'code>>,
    registers: Vec<Value<'code>>
}


impl<'code> EvaluationContext<'code> {
    pub fn run_command(&mut self, command: &[Atom<'code>]) {
        if command.is_empty() { return; }
        let head = command.first().unwrap();
        let body = &command[1..];
        if self.scopes.is_empty() { self.scopes.push(Object::default()); }
        let count = self.scopes.len();
        match head {
            Atom::WORD(d, _) => {
                let word = String::from(*d);
                for scope in self.scopes.clone().iter_mut().rev() {
                    let optional_value = scope.content.remove(&word);
                    if optional_value.is_none() { panic!("Undefined identifier as the head of a command. "); }
                    match optional_value.unwrap() {
                        Value::FUNCTION(function) => {
                            self.registers.clear();
                            let value = function.call(self, body);
                            self.registers.push(value);
                        },
                        Value::OBJECT(object) => {
                            self.scopes.push(object);
                            self.run_command(body);
                            self.registers.clear();
                            self.registers.push(Value::OBJECT(self.scopes.pop().unwrap()));
                        },
                        _ => {
                            panic!("Unexpected value as the head of a command.");
                        }
                    }
                }
            },
            _ => unreachable!("Non-word as the head of a command should be unreachable.")
        }
        if self.scopes.len() != count { 
            unreachable!("Unequal scope count after evaluation with custom object as scope should be unreachable."); 
        }
    }

    pub fn run_code(&mut self, code: &'code String) {
        let result = lex(code);
        if result.is_err() {
            let error = result.unwrap_err();
            panic!("Lexical analysis error: {} [line {}, column {}]", error.message, error.position.0, error.position.1);
        }

        let result = generate_commands(&result.unwrap());
        if result.is_err() {
            let error = result.unwrap_err();
            panic!("Command generation error: {} [line {}, column {}]", error.message, error.position.0, error.position.1);
        }

        for command in result.unwrap().iter() { self.run_command(command); }

    }
}

