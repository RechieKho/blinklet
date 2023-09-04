use super::{object::Object, value::Value};
use crate::parser::{lexer::lex, command::{generate_commands, Atom}};

#[derive(Default, Clone)]
pub struct EvaluationContext{
    scopes: Vec<Object>,
    registers: Vec<Value>
}


impl EvaluationContext {
    pub fn run_command(&mut self, command: &[Atom]) {
        if command.is_empty() { return; }
        let head = command.first().unwrap();
        let body = &command[1..];
        if self.scopes.is_empty() { self.scopes.push(Object::default()); }
        match head {
            Atom::WORD(d, _) => {
                for scope in self.scopes.iter().as_slice().to_vec().iter().rev() {
                    let optional_value = scope.get(&String::from(*d));
                    if optional_value.is_none() { panic!("Undefined identifier as the head of a command. "); }
                    match optional_value.unwrap() {
                        Value::FUNCTION(function) => {
                            self.registers.clear();
                            let value = function.call(self, body);
                            self.registers.push(value);
                            return; 
                        },
                        Value::OBJECT(object) => {
                            self.scopes.push(object.clone());
                            let count = self.scopes.len();
                            self.run_command(body);
                            if self.scopes.len() != count { unreachable!("Unequal scope count after evaluation with custom object as scope should be unreachable."); }
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
    }

    pub fn run_code(&mut self, code: &String) {
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

