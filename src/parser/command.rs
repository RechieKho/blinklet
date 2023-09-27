use super::lexer::Token;
use super::lexer::TokenLine;
use super::lexer::TokenValue;
use crate::backtrace::Backtrace;
use crate::mark::Mark;
use crate::raise_bug;
use crate::raise_error;
use std::rc::Rc;

const NULL_STR: &'static str = "null";
const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

#[macro_export]
macro_rules! atom_as_identifier {
    ($atom: expr) => {
        if let AtomValue::IDENTIFIER(ref identifier) = $atom.value {
            identifier
        } else {
            raise_error!(Some($atom.mark.clone()), "Expecting an identifier.");
        }
    };
}

#[macro_export]
macro_rules! atom_as_command {
    ($atom: expr) => {
        if let AtomValue::COMMAND(ref command) = $atom.value {
            command
        } else {
            raise_error!(Some($atom.mark.clone()), "Expecting a command.");
        }
    };
}

#[derive(Debug, Clone)]
pub enum AtomValue {
    NULL,
    IDENTIFIER(String),
    BOOL(bool),
    STRING(String),
    NUMBER(f64),
    COMMAND(Vec<Atom>),
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub value: AtomValue,
    pub mark: Rc<Mark>,
}

impl Atom {
    pub fn new_null(mark: Rc<Mark>) -> Self {
        Atom {
            value: AtomValue::NULL,
            mark,
        }
    }

    pub fn new_identifier(identifier: String, mark: Rc<Mark>) -> Self {
        Atom {
            value: AtomValue::IDENTIFIER(identifier),
            mark,
        }
    }

    pub fn new_bool(boolean: bool, mark: Rc<Mark>) -> Self {
        Atom {
            value: AtomValue::BOOL(boolean),
            mark,
        }
    }

    pub fn new_string(string: String, mark: Rc<Mark>) -> Self {
        Atom {
            value: AtomValue::STRING(string),
            mark,
        }
    }

    pub fn new_number(number: f64, mark: Rc<Mark>) -> Self {
        Atom {
            value: AtomValue::NUMBER(number),
            mark,
        }
    }

    pub fn new_command(command: Vec<Atom>, mark: Rc<Mark>) -> Self {
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
                    Atom::new_null(mark)
                } else if word == TRUE_STR {
                    Atom::new_bool(true, mark)
                } else if word == FALSE_STR {
                    Atom::new_bool(false, mark)
                } else {
                    Atom::new_identifier(word, mark)
                }
            }
            TokenValue::STRING(string) => Atom::new_string(string, mark),
            TokenValue::NUMBER(number) => Atom::new_number(number, mark),
        }
    }
}

pub fn generate_commands(mut lot: Vec<TokenLine>) -> Result<Vec<Atom>, Backtrace> {
    let mut result: Vec<Atom> = Vec::new();
    let mut current_indent_count = 0usize;

    fn get_subatom_mut(atom: &mut Atom, nesting: usize) -> Option<&mut Atom> {
        if nesting == 0 {
            return if let AtomValue::COMMAND(_) = atom.value {
                Some(atom)
            } else {
                None
            };
        }

        if let AtomValue::COMMAND(ref mut command) = atom.value {
            let last = command.last_mut()?;
            return get_subatom_mut(last, nesting - 1);
        } else {
            return None;
        }
    }

    for mut token_line in lot.drain(..) {
        let indent_displacement = token_line.indent_count as isize - current_indent_count as isize;
        if indent_displacement > 1 {
            raise_error!(
                Some(Rc::new(Mark::new(token_line.mark_line, 0..=0))),
                "Excessive indentation."
            );
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
            raise_error!(
                Some(Rc::new(Mark::new(token_line.mark_line, 0..=0))),
                "Unexpected indentation."
            );
        }

        // Just append to the result since there is no indentation.
        if token_line.indent_count == 0 {
            result.push(Atom::new_command(
                atoms,
                Rc::new(Mark::new(
                    token_line.mark_line.clone(),
                    0..=token_line.mark_line.content.len(),
                )),
            ));
            current_indent_count = token_line.indent_count;
            continue;
        }

        // There is indentation, get the parent command and push the subcommand.
        let parent_atom =
            get_subatom_mut(result.last_mut().unwrap(), token_line.indent_count - 1).unwrap();

        let parent_command = if let AtomValue::COMMAND(ref mut command) = parent_atom.value {
            command
        } else {
            raise_error!(Some(parent_atom.mark.clone()), "Expecting a command.");
        };

        {
            let first_atom = atoms.first().unwrap();

            match first_atom.value {
                AtomValue::IDENTIFIER(ref identifier) => {
                    if identifier == "|" {
                        atoms.remove(0); // Remove the "|".
                        parent_command.append(&mut atoms);
                        current_indent_count = token_line.indent_count;
                        continue;
                    }
                }
                AtomValue::STRING(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "String as the head of a command is forbidden."
                    );
                }
                AtomValue::NUMBER(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "Number as the head of a command is forbidden."
                    );
                }
                AtomValue::BOOL(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "Bool as the head of a command is forbidden."
                    );
                }
                AtomValue::NULL => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "Null as the head of a command is forbidden."
                    );
                }
                AtomValue::COMMAND(_) => {
                    raise_bug!(
                        Some(first_atom.mark.clone()),
                        "Command as the head of a command should be unreachable."
                    );
                }
            }
        }
        parent_command.push(Atom::new_command(
            atoms,
            Rc::new(Mark::new(
                token_line.mark_line.clone(),
                0..=token_line.mark_line.content.len(),
            )),
        ));
        current_indent_count = token_line.indent_count;
    }

    Ok(result)
}
