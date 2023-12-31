use super::token::Token;
use super::token::TokenLine;
use super::token::TokenValue;
use crate::backtrace::Backtrace;
use crate::mark::Mark;
use crate::raise_bug;
use crate::raise_error;

const NULL_STR: &'static str = "null";
const TRUE_STR: &'static str = "true";
const FALSE_STR: &'static str = "false";

#[macro_export]
macro_rules! atom_as_identifier {
    ($atom: expr) => {
        if let crate::parser::atom::AtomValue::IDENTIFIER(ref identifier) = $atom.value {
            identifier
        } else {
            crate::raise_error!(Some($atom.mark.clone()), "Expecting an identifier.");
        }
    };
}

#[macro_export]
macro_rules! atom_as_statement {
    ($atom: expr) => {
        if let crate::parser::atom::AtomValue::STATEMENT(ref statement) = $atom.value {
            statement
        } else {
            crate::raise_error!(Some($atom.mark.clone()), "Expecting a statement.");
        }
    };
}

#[derive(Debug, Clone)]
pub enum AtomValue {
    NULL,
    IDENTIFIER(String),
    BOOL(bool),
    STRING(String),
    FLOAT(f64),
    STATEMENT(Vec<Atom>),
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub value: AtomValue,
    pub mark: Mark,
}

impl Atom {
    pub fn new_null(mark: Mark) -> Self {
        Atom {
            value: AtomValue::NULL,
            mark,
        }
    }

    pub fn new_identifier(identifier: String, mark: Mark) -> Self {
        Atom {
            value: AtomValue::IDENTIFIER(identifier),
            mark,
        }
    }

    pub fn new_bool(boolean: bool, mark: Mark) -> Self {
        Atom {
            value: AtomValue::BOOL(boolean),
            mark,
        }
    }

    pub fn new_string(string: String, mark: Mark) -> Self {
        Atom {
            value: AtomValue::STRING(string),
            mark,
        }
    }

    pub fn new_float(float: f64, mark: Mark) -> Self {
        Atom {
            value: AtomValue::FLOAT(float),
            mark,
        }
    }

    pub fn new_statement(statement: Vec<Atom>, mark: Mark) -> Self {
        Atom {
            value: AtomValue::STATEMENT(statement),
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
            TokenValue::FLOAT(float) => Atom::new_float(float, mark),
        }
    }
}

pub fn generate_statements(mut lot: Vec<TokenLine>) -> Result<Vec<Atom>, Backtrace> {
    let mut result: Vec<Atom> = Vec::new();
    let mut current_indent_count = 0usize;

    fn get_subatom_mut(atom: &mut Atom, nesting: usize) -> Option<&mut Atom> {
        if nesting == 0 {
            return if let AtomValue::STATEMENT(_) = atom.value {
                Some(atom)
            } else {
                None
            };
        }

        if let AtomValue::STATEMENT(ref mut statement) = atom.value {
            let last = statement.last_mut()?;
            return get_subatom_mut(last, nesting - 1);
        } else {
            return None;
        }
    }

    for mut token_line in lot.drain(..) {
        let indent_displacement = token_line.indent_count as isize - current_indent_count as isize;
        if indent_displacement > 1 {
            raise_error!(
                Some(Mark::new(token_line.mark_line, 0..=0)),
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

        // Indentation at the very first statement, this is a sin.
        if result.len() == 0 && token_line.indent_count != 0 {
            raise_error!(
                Some(Mark::new(token_line.mark_line, 0..=0)),
                "Unexpected indentation."
            );
        }

        // Just append to the result since there is no indentation.
        if token_line.indent_count == 0 {
            result.push(Atom::new_statement(
                atoms,
                Mark::new(
                    token_line.mark_line.clone(),
                    0..=token_line.mark_line.content.len(),
                ),
            ));
            current_indent_count = token_line.indent_count;
            continue;
        }

        // There is indentation, get the parent statement and push the substatement.
        let parent_atom =
            get_subatom_mut(result.last_mut().unwrap(), token_line.indent_count - 1).unwrap();

        let parent_statement = if let AtomValue::STATEMENT(ref mut statement) = parent_atom.value {
            statement
        } else {
            raise_error!(Some(parent_atom.mark.clone()), "Expecting a statement.");
        };

        {
            let first_atom = atoms.first().unwrap();

            match first_atom.value {
                AtomValue::IDENTIFIER(ref identifier) => {
                    if identifier == "|" {
                        atoms.remove(0); // Remove the "|".
                        parent_statement.append(&mut atoms);
                        current_indent_count = token_line.indent_count;
                        continue;
                    }
                }
                AtomValue::STRING(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "String as the head of a statement is forbidden."
                    );
                }
                AtomValue::FLOAT(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "FLOAT as the head of a statement is forbidden."
                    );
                }
                AtomValue::BOOL(_) => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "Bool as the head of a statement is forbidden."
                    );
                }
                AtomValue::NULL => {
                    raise_error!(
                        Some(first_atom.mark.clone()),
                        "Null as the head of a statement is forbidden."
                    );
                }
                AtomValue::STATEMENT(_) => {
                    raise_bug!(
                        Some(first_atom.mark.clone()),
                        "Statement as the head of a statement should be unreachable."
                    );
                }
            }
        }
        parent_statement.push(Atom::new_statement(
            atoms,
            Mark::new(
                token_line.mark_line.clone(),
                0..=token_line.mark_line.content.len(),
            ),
        ));
        current_indent_count = token_line.indent_count;
    }

    Ok(result)
}
