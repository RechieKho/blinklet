use super::lexer::Token;
use super::lexer::TokenLine;
use super::lexer::TokenValue;
use crate::log::Log;
use crate::mark::Mark;
use std::rc::Rc;

const NULL_STR: &'static str = "null";
const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

pub type Command = Vec<Atom>;

#[derive(Debug, Clone)]
pub enum AtomValue {
    NULL,
    IDENTIFIER(String),
    BOOL(bool),
    STRING(String),
    NUMBER(f64),
    COMMAND(Command),
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub value: AtomValue,
    pub mark: Option<Rc<Mark>>,
}

impl Atom {
    pub fn new_null(mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::NULL,
            mark,
        }
    }

    pub fn new_identifier(identifier: String, mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::IDENTIFIER(identifier),
            mark,
        }
    }

    pub fn new_bool(boolean: bool, mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::BOOL(boolean),
            mark,
        }
    }

    pub fn new_string(string: String, mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::STRING(string),
            mark,
        }
    }

    pub fn new_number(number: f64, mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::NUMBER(number),
            mark,
        }
    }

    pub fn new_command(command: Command, mark: Option<Rc<Mark>>) -> Self {
        Atom {
            value: AtomValue::COMMAND(command),
            mark,
        }
    }

    pub fn from_token(token: Token) -> Self {
        let Token { value, mark } = token;
        match value {
            TokenValue::WORD(word) => {
                if word == NULL_STR {
                    Atom::new_null(Some(mark))
                } else if word == TRUE_STR {
                    Atom::new_bool(true, Some(mark))
                } else if word == FALSE_STR {
                    Atom::new_bool(false, Some(mark))
                } else {
                    Atom::new_identifier(word, Some(mark))
                }
            }
            TokenValue::STRING(string) => Atom::new_string(string, Some(mark)),
            TokenValue::NUMBER(number) => Atom::new_number(number, Some(mark)),
        }
    }
}

pub fn generate_commands(mut lot: Vec<TokenLine>) -> Result<Vec<Command>, Log> {
    let mut result: Vec<Command> = Vec::new();
    let mut current_indent_count = 0usize;

    fn get_subcommand_mut<'command, 'pool>(
        command: &'command mut Command,
        nesting: usize,
    ) -> Option<&'command mut Command> {
        let mut subcommand = command;
        for _ in 0..nesting {
            let last = subcommand.last_mut();
            if last.is_none() {
                return None;
            }
            let atom = last.unwrap();
            if let AtomValue::COMMAND(ref mut c) = atom.value {
                subcommand = c;
            } else {
                return None;
            }
        }
        Some(subcommand)
    }

    for mut token_line in lot.drain(..) {
        let indent_displacement = token_line.indent_count as isize - current_indent_count as isize;
        if indent_displacement > 1 {
            return Err(Log::error(
                format!("Excessive indentation."),
                Some(Rc::new(Mark::new(token_line.mark_line, 0..0))),
            ));
        }

        let mut atoms: Vec<Atom> = Vec::default();
        for token in token_line.tokens.drain(..) {
            // Collect atoms.
            let new_atom: Atom = Atom::from_token(token);
            atoms.push(new_atom);
        }

        if atoms.len() == 0 {
            current_indent_count = token_line.indent_count;
            continue;
        }

        // Indentation at the very first command, this is a sin.
        if result.len() == 0 && token_line.indent_count != 0 {
            return Err(Log::error(
                format!("Unexpected indentation."),
                Some(Rc::new(Mark::new(token_line.mark_line, 0..0))),
            ));
        }

        // Just append to the result since there is no indentation.
        if token_line.indent_count == 0 {
            result.push(atoms);
            current_indent_count = token_line.indent_count;
            continue;
        }
        // There is indentation, get the parent command and push the subcommand.
        let parent_command =
            get_subcommand_mut(result.last_mut().unwrap(), token_line.indent_count - 1).unwrap();
        {
            let first_atom = atoms.first().unwrap();

            match first_atom.value {
                AtomValue::IDENTIFIER(ref identifier) => {
                    if identifier == "ensuing" {
                        parent_command.append(&mut atoms);
                        current_indent_count = token_line.indent_count;
                        continue;
                    }
                }
                AtomValue::STRING(_) => {
                    return Err(Log::error(
                        format!("String as the head of a command is forbidden."),
                        first_atom.mark.clone(),
                    ));
                }
                AtomValue::NUMBER(_) => {
                    return Err(Log::error(
                        format!("Number as the head of a command is forbidden."),
                        first_atom.mark.clone(),
                    ));
                }
                AtomValue::BOOL(_) => {
                    return Err(Log::error(
                        format!("Bool as the head of a command is forbidden."),
                        first_atom.mark.clone(),
                    ));
                }
                AtomValue::NULL => {
                    return Err(Log::error(
                        format!("Null as the head of a command is forbidden."),
                        first_atom.mark.clone(),
                    ));
                }
                AtomValue::COMMAND(_) => {
                    unreachable!("Command as the head of a command should be unreachable.");
                }
            }
        }
        parent_command.push(Atom::new_command(
            atoms,
            Some(Rc::new(Mark::new(
                token_line.mark_line.clone(),
                0..token_line.mark_line.content.len(),
            ))),
        ));
        current_indent_count = token_line.indent_count;
    }

    Ok(result)
}
