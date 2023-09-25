use crate::backtrace::Backtrace;
use crate::mark::{Mark, MarkLine};
use crate::raise_error;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    WORD(String),
    STRING(String),
    NUMBER(f64),
}

#[derive(Debug)]
pub struct Token {
    pub value: TokenValue,
    pub mark: Rc<Mark>,
}

impl Token {
    pub fn new_word(word: String, mark_line: Rc<MarkLine>, column: RangeInclusive<usize>) -> Self {
        Token {
            value: TokenValue::WORD(word),
            mark: Rc::new(Mark::new(mark_line, column)),
        }
    }

    pub fn new_string(
        string: String,
        mark_line: Rc<MarkLine>,
        column: RangeInclusive<usize>,
    ) -> Self {
        let formatted = string.replace("\\n", "\n").replace("\\\\", "\\");

        Token {
            value: TokenValue::STRING(formatted),
            mark: Rc::new(Mark::new(mark_line, column)),
        }
    }

    pub fn new_number(number: f64, mark_line: Rc<MarkLine>, column: RangeInclusive<usize>) -> Self {
        Token {
            value: TokenValue::NUMBER(number),
            mark: Rc::new(Mark::new(mark_line, column)),
        }
    }
}

#[derive(Debug)]
pub struct TokenLine {
    pub mark_line: Rc<MarkLine>,
    pub tokens: Vec<Token>,
    pub indent_count: usize,
}

pub fn lex(name: String, code: String) -> Result<Vec<TokenLine>, Backtrace> {
    let name = Rc::new(name);
    let mut result: Vec<TokenLine> = Vec::new();
    let mut indent_char = '\0';
    let mut indent_factor = 0usize;

    'row: for (i, line) in code.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let mark_line = Rc::new(MarkLine::new(name.clone(), Rc::new(String::from(line)), i));

        let mut token_line: TokenLine = TokenLine {
            mark_line: mark_line.clone(),
            tokens: Vec::new(),
            indent_count: 0,
        };
        let mut is_indent_scanned = false;
        let mut string_char = '\0';
        let mut slice_start = 0usize;

        'column: for (j, current_char) in line.chars().into_iter().enumerate() {
            // Collect indentation count.
            if !is_indent_scanned {
                if current_char.is_whitespace() {
                    if indent_char == '\0' {
                        indent_char = current_char;
                    }
                    if current_char != indent_char {
                        raise_error!(
                            Some(Rc::new(Mark::new(mark_line, 0..=j))),
                            "Inconsistent indentation character."
                        );
                    }
                    token_line.indent_count += 1;
                    continue;
                } else {
                    is_indent_scanned = true;
                    slice_start = j;
                    if indent_factor == 0 {
                        indent_factor = token_line.indent_count;
                    }
                    if indent_factor != 0 {
                        // We are not using else to consider the value change.
                        if token_line.indent_count % indent_factor != 0 {
                            raise_error!(
                                Some(Rc::new(Mark::new(mark_line, 0..=j))),
                                "Inconsistent indentation factor."
                            );
                        }
                        token_line.indent_count /= indent_factor
                    }
                }
            }

            // Check if it is in string literal
            if string_char != '\0' {
                if current_char == string_char {
                    token_line.tokens.push(Token::new_string(
                        String::from(&line[slice_start..j]),
                        mark_line.clone(),
                        slice_start..=j,
                    ));
                    slice_start = j + 1;
                    string_char = '\0';
                }
                continue;
            }

            // Check if it is a comment.
            if current_char == '#' {
                break 'row;
            }

            // Check if it is starting a string literal.
            if current_char == '\'' {
                string_char = current_char;
                if slice_start != j {
                    let slice = &line[slice_start..j];
                    let parse_result = slice.parse::<f64>();
                    if parse_result.is_ok() {
                        token_line.tokens.push(Token::new_number(
                            parse_result.unwrap(),
                            mark_line.clone(),
                            slice_start..=j,
                        ));
                    } else {
                        token_line.tokens.push(Token::new_word(
                            String::from(slice),
                            mark_line.clone(),
                            slice_start..=j,
                        ));
                    }
                }
                slice_start = j + 1;
                continue;
            }

            // Check if it is a whitespace.
            if current_char.is_whitespace() {
                if slice_start != j {
                    let slice = &line[slice_start..j];
                    let parse_result = slice.parse::<f64>();
                    if parse_result.is_ok() {
                        token_line.tokens.push(Token::new_number(
                            parse_result.unwrap(),
                            mark_line.clone(),
                            slice_start..=j,
                        ));
                    } else {
                        token_line.tokens.push(Token::new_word(
                            String::from(slice),
                            mark_line.clone(),
                            slice_start..=j,
                        ));
                    }
                }
                slice_start = j + 1;
                continue;
            }
        }

        let line_length = line.len();

        // Check if unterminated string literal.
        if string_char != '\0' {
            raise_error!(
                Some(Rc::new(Mark::new(
                    mark_line,
                    slice_start..=(line_length - 1),
                ))),
                "unterminated string."
            );
        }

        // Check if there is unhandled token.
        if slice_start != line_length {
            let slice = &line[slice_start..line_length];
            let parse_result = slice.parse::<f64>();
            if parse_result.is_ok() {
                token_line.tokens.push(Token::new_number(
                    parse_result.unwrap(),
                    mark_line.clone(),
                    slice_start..=line_length,
                ));
            } else {
                token_line.tokens.push(Token::new_word(
                    String::from(slice),
                    mark_line.clone(),
                    slice_start..=line_length,
                ));
            }
        }

        // Push `TokenLine`.
        result.push(token_line);
    }

    Ok(result)
}
