use super::lexer::Line;
use super::lexer::Token;
use super::lexer::TokenValue;
use crate::error::Error;
use crate::mark::Mark;
use std::ops::Range;

const NULL_STR: &'static str = "null";
const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

pub type Command<'name, 'code> = Vec<Atom<'name, 'code>>;

#[derive(Debug, Clone)]
pub enum AtomValue<'name, 'code> {
    NULL,
    IDENTIFIER(&'code str),
    BOOL(bool),
    STRING(&'code str),
    NUMBER(f64),
    COMMAND(Command<'name, 'code>),
}

#[derive(Debug, Clone)]
pub struct Atom<'name, 'code> {
    pub value: AtomValue<'name, 'code>,
    pub mark: Option<Mark<'name, 'code>>,
}

impl<'name, 'code> Atom<'name, 'code> {
    pub fn new_marked_null(
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::NULL,
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn new_marked_identifier(
        identifier: &'code str,
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::IDENTIFIER(identifier),
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn new_marked_bool(
        boolean: bool,
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::BOOL(boolean),
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn new_marked_string(
        string: &'code str,
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::STRING(string),
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn new_marked_number(
        number: f64,
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::NUMBER(number),
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn new_marked_command(
        command: Command<'name, 'code>,
        name: &'name String,
        row: usize,
        column: Range<usize>,
        line: &'code str,
    ) -> Self {
        Atom {
            value: AtomValue::COMMAND(command),
            mark: Some(Mark {
                name,
                row,
                column,
                line,
            }),
        }
    }

    pub fn from_token(token: Token<'name, 'code>) -> Self {
        let Token { value, mark } = token;
        let Mark {
            name,
            row,
            column,
            line,
        } = mark;
        match value {
            TokenValue::WORD(word) => {
                if word == NULL_STR {
                    Atom::new_marked_null(name, row, column, line)
                } else if word == TRUE_STR {
                    Atom::new_marked_bool(true, name, row, column, line)
                } else if word == FALSE_STR {
                    Atom::new_marked_bool(false, name, row, column, line)
                } else {
                    Atom::new_marked_identifier(word, name, row, column, line)
                }
            }
            TokenValue::STRING(string) => Atom::new_marked_string(string, name, row, column, line),
            TokenValue::NUMBER(number) => Atom::new_marked_number(number, name, row, column, line),
        }
    }
}

pub fn generate_commands<'name, 'code>(
    mut lot: Vec<Line<'name, 'code>>,
) -> Result<Vec<Command<'name, 'code>>, Error<'name, 'code>> {
    let mut result: Vec<Command<'name, 'code>> = Vec::new();
    let mut current_indent_count = 0usize;

    fn get_subcommand_mut<'command, 'name, 'code>(
        command: &'command mut Command<'name, 'code>,
        nesting: usize,
    ) -> Option<&'command mut Command<'name, 'code>> {
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

    for mut line in lot.drain(..) {
        let indent_displacement = line.indent_count as isize - current_indent_count as isize;
        if indent_displacement > 1 {
            return Err(Error {
                message: format!("Excessive indentation."),
                mark: Some(Mark {
                    name: line.name,
                    row: line.row,
                    column: 0..0,
                    line: line.line,
                }),
            });
        }

        let mut atoms: Vec<Atom<'name, 'code>> = Vec::default();
        for token in line.tokens.drain(..) {
            // Collect atoms.
            let new_marked_atom: Atom<'name, 'code> = Atom::from_token(token);
            atoms.push(new_marked_atom);
        }

        if atoms.len() == 0 {
            current_indent_count = line.indent_count;
            continue;
        }

        // Indentation at the very first command, this is a sin.
        if result.len() == 0 && line.indent_count != 0 {
            return Err(Error {
                message: format!("Unexpected indentation."),
                mark: Some(Mark {
                    name: line.name,
                    row: line.row,
                    column: 0..0,
                    line: line.line,
                }),
            });
        }

        // Just append to the result since there is no indentation.
        if line.indent_count == 0 {
            result.push(atoms);
            current_indent_count = line.indent_count;
            continue;
        }
        // There is indentation, get the parent command and push the subcommand.
        let parent_command =
            get_subcommand_mut(result.last_mut().unwrap(), line.indent_count - 1).unwrap();
        {
            let first_atom = atoms.first().unwrap();

            match first_atom.value {
                AtomValue::IDENTIFIER(identifier) => {
                    if identifier == "ensuing" {
                        parent_command.append(&mut atoms);
                        current_indent_count = line.indent_count;
                        continue;
                    }
                }
                AtomValue::STRING(_) => {
                    return Err(Error {
                        message: format!("String as the head of a command is forbidden."),
                        mark: first_atom.mark.clone(),
                    });
                }
                AtomValue::NUMBER(_) => {
                    return Err(Error {
                        message: format!("Number as the head of a command is forbidden."),
                        mark: first_atom.mark.clone(),
                    });
                }
                AtomValue::BOOL(_) => {
                    return Err(Error {
                        message: format!("Bool as the head of a command is forbidden."),
                        mark: first_atom.mark.clone(),
                    });
                }
                AtomValue::NULL => {
                    return Err(Error {
                        message: format!("Null as the head of a command is forbidden."),
                        mark: first_atom.mark.clone(),
                    });
                }
                AtomValue::COMMAND(_) => {
                    unreachable!("Command as the head of a command should be unreachable.");
                }
            }
        }
        parent_command.push(Atom::new_marked_command(
            atoms,
            line.name,
            line.row,
            0..line.line.len(),
            line.line,
        ));
        current_indent_count = line.indent_count;
    }

    Ok(result)
}
