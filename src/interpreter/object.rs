use super::{function::NativeFunction, standard::greet, value::Value};
use crate::parser::{
    command::{generate_commands, Atom},
    lexer::lex,
};
use std::{collections::HashMap, mem};

const PARENT_KEY: &'static str = "parent";
const RETURN_KEY: &'static str = "return";

#[derive(Clone)]
pub struct Object<'code> {
    pub content: HashMap<String, Value<'code>>,
}

impl<'code> ToString for Object<'code> {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl<'code> Default for Object<'code> {
    fn default() -> Object<'code> {
        let mut object = Object {
            content: HashMap::default(),
        };
        object
            .content
            .insert(String::from("greet"), NativeFunction::wrap(greet));
        object
    }
}

impl<'code> Object<'code> {
    pub fn push(&mut self, object: Object<'code>) {
        let parent = mem::replace(self, object);
        self.content
            .insert(String::from(PARENT_KEY), Value::OBJECT(parent));
    }

    pub fn pop(&mut self) -> Option<Object<'code>> {
        let optional_parent = self.content.get_mut(&String::from(PARENT_KEY));
        if optional_parent.is_some() {
            if let Value::OBJECT(ref mut object) = optional_parent.unwrap() {
                let taken_object = mem::take(object);
                return Some(mem::replace(self, taken_object));
            }
        }
        return None;
    }

    pub fn run_command(&mut self, command: &[Atom<'code>]) {
        if command.is_empty() {
            return;
        }
        let head = command.first().unwrap();
        let body = &command[1..];
        match head {
            Atom::IDENTIFIER(d, _) => {
                let k = String::from(*d);
                let optional_value = self.content.get(&k);
                if optional_value.is_none() {
                    let optional_parent = self.content.get_mut(&String::from(PARENT_KEY));
                    if optional_parent.is_some() {
                        if let Value::OBJECT(ref mut object) = optional_parent.unwrap() {
                            object.run_command(command);
                        }
                    }
                    return;
                }

                match optional_value.unwrap().clone() {
                    Value::FUNCTION(function) => {
                        self.push(Object::default());
                        let v = function.call(self, body);
                        self.pop();
                        self.content.insert(String::from(RETURN_KEY), v);
                    }
                    Value::OBJECT(object) => {
                        self.push(object);
                        self.run_command(body);
                        let object = self.pop().unwrap();
                        self.content
                            .insert(String::from(RETURN_KEY), Value::OBJECT(object));
                    }
                    _ => panic!("Unexpected value as the head of a command."),
                }
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
