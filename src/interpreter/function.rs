use super::object::Object;
use super::value::Value;
use crate::parser::command::Atom;
use crate::parser::command::AtomValue;
use std::sync::Arc;

pub const RETURN_KEY: &'static str = "return";
pub const RESULT_KEY: &'static str = "result";
pub const ERROR_KEY: &'static str = "error";

pub trait Function<'code>: ToString {
    fn call(&self, context: &mut Object<'code>, body: &[Atom<'code>]);
}

pub struct ScriptFunction<'code> {
    pub command: Vec<Atom<'code>>,
}

pub type NativeFunctionHandler<'code> = fn(context: &mut Object<'code>, body: &[Atom<'code>]);

pub struct NativeFunction<'code> {
    pub handler: NativeFunctionHandler<'code>,
}

impl<'code> ToString for ScriptFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Script function>")
    }
}

impl<'code> Function<'code> for ScriptFunction<'code> {
    fn call(&self, context: &mut Object<'code>, body: &[Atom<'code>]) {
        let mut argument_index = 1usize;
        for atom in self.command.iter() {
            match atom.value {
                AtomValue::IDENTIFIER(parameter_identifier) => {
                    if argument_index >= body.len() {
                        continue;
                    }
                    let argument_atom = &body[argument_index];
                    let argument = match argument_atom.value {
                        AtomValue::IDENTIFIER(argument_identifier) => context
                            .content
                            .get(argument_identifier)
                            .unwrap_or(&Value::NULL)
                            .clone(),
                        AtomValue::BOOL(boolean) => Value::BOOL(boolean),
                        AtomValue::NULL => Value::NULL,
                        AtomValue::STRING(string) => Value::STRING(String::from(string)),
                        AtomValue::NUMBER(number) => Value::NUMBER(number),
                        AtomValue::COMMAND(ref command) => {
                            context.push(Object::default());
                            let result = context.run_command(command.as_slice());
                            let mut object = context.pop().unwrap();
                            if result.is_err() {
                                context.content.insert(
                                    String::from(ERROR_KEY),
                                    Value::STRING(String::from(result.unwrap_err().message)),
                                );
                                return;
                            }
                            if object.content.contains_key(RESULT_KEY) {
                                object.content.remove(RESULT_KEY).unwrap_or(Value::NULL)
                            } else {
                                Value::NULL
                            }
                        }
                    };
                    context
                        .content
                        .insert(String::from(parameter_identifier), argument);
                    argument_index += 1;
                }

                AtomValue::COMMAND(ref command) => {
                    let result = context.run_command(command);
                    if result.is_err() {
                        context.content.insert(
                            String::from(ERROR_KEY),
                            Value::STRING(String::from(result.unwrap_err().message)),
                        );
                        return;
                    }
                    if context.content.contains_key(RETURN_KEY) {
                        return;
                    }
                }

                _ => {
                    context.content.insert(
                        String::from(ERROR_KEY),
                        Value::STRING(String::from("Unexpected value as the head of a command.")),
                    );
                    return;
                }
            }
        }
    }
}

impl<'code> ScriptFunction<'code> {
    pub fn wrap(command: &[Atom<'code>]) -> Value<'code> {
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(ScriptFunction {
            command: command.to_vec(),
        });
        Value::FUNCTION(function)
    }
}

impl<'code> ToString for NativeFunction<'code> {
    fn to_string(&self) -> String {
        format!("<Native function at {:p}>", self)
    }
}

impl<'code> Function<'code> for NativeFunction<'code> {
    fn call(&self, context: &mut Object<'code>, body: &[Atom<'code>]) {
        (self.handler)(context, body)
    }
}

impl<'code> NativeFunction<'code> {
    pub fn wrap(handler: NativeFunctionHandler<'code>) -> Value<'code> {
        let function: Arc<dyn Function<'code> + 'code> = Arc::new(NativeFunction { handler });
        Value::FUNCTION(function)
    }
}
