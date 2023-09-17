use std::result::Result;
use std::string::String;
use std::vec::Vec;
use std::ops::Range;
use crate::error::Error;
use crate::mark::Mark;

#[derive(Debug, PartialEq)]
pub enum TokenValue<'code> {
    WORD(&'code str),
    STRING(&'code str),
    NUMBER(f64),
}

#[derive(Debug)]
pub struct Token<'code> {
    pub value: TokenValue<'code>,
    pub mark: Mark<'code>
}

impl<'code> Token<'code> {
    pub fn new_word(word: &'code str, row: usize, column: Range<usize>, line: &'code str) -> Self {
        Token {
            value: TokenValue::WORD(word),
            mark: Mark { row, column, line }
        }
    }

    pub fn new_string(string: &'code str, row: usize, column: Range<usize>, line: &'code str) -> Self {
        Token {
            value: TokenValue::STRING(string),
            mark: Mark { row, column, line }
        }
    }

    pub fn new_number(number: f64, row: usize, column: Range<usize>, line: &'code str) -> Self {
        Token {
            value: TokenValue::NUMBER(number),
            mark: Mark { row, column, line }
        }
    }
}

#[derive(Debug, Default)]
pub struct Line<'code> {
    pub tokens: Vec<Token<'code>>,
    pub indent_count: usize,
    pub row: usize,
    pub line: &'code str,
}

pub fn lex<'code>(code: &'code String) -> Result<Vec<Line<'code>>, Error<'code>> {
    let mut result: Vec<Line<'code>> = Vec::new();
    let mut indent_char = '\0';
    let mut indent_factor = 0usize;
    for (i, line) in code.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let mut line_result: Line<'code> = Line::default();
        let mut is_indent_scanned = false;
        let mut string_char = '\0';
        let mut slice_start = 0usize;

        line_result.row = i;
        line_result.line = line;

        for (j, current_char) in line.chars().into_iter().enumerate() {
            // Collect indentation count.
            if !is_indent_scanned {
                if current_char.is_whitespace() {
                    if indent_char == '\0' {
                        indent_char = current_char;
                    }
                    if current_char != indent_char {
                        return Err(Error {
                            message: "Inconsistent indentation character.",
                            mark: Mark {
                                row: i,
                                column: 0..j,
                                line
                            }
                        });
                    }
                    line_result.indent_count += 1;
                    continue;
                } else {
                    is_indent_scanned = true;
                    slice_start = j;
                    if indent_factor == 0 {
                        indent_factor = line_result.indent_count;
                    }
                    if indent_factor != 0 {
                        // We are not using else to consider the value change.
                        if line_result.indent_count % indent_factor != 0 {
                            return Err(Error {
                                message: "Inconsistent indentation factor.",
                                mark: Mark {
                                    row: i,
                                    column: 0..j,
                                    line
                                }
                            });
                        }
                        line_result.indent_count /= indent_factor
                    }
                }
            }

            // Check if it is in string literal
            if string_char != '\0' {
                if current_char == string_char {
                    line_result
                        .tokens
                        .push(Token::new_string(&line[slice_start..j], i, slice_start..j, line));
                    slice_start = j + 1;
                    string_char = '\0';
                }
                continue;
            }

            // Check if it is starting a string literal.
            if current_char == '\'' {
                string_char = current_char;
                if slice_start != j {
                    let slice = &line[slice_start..j];
                    let parse_result = slice.parse::<f64>();
                    if parse_result.is_ok() {
                        line_result
                            .tokens
                            .push(Token::new_number(parse_result.unwrap(), i, slice_start..j, line));
                    } else {
                        line_result.tokens.push(Token::new_word(slice, i, slice_start..j, line));
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
                        line_result
                            .tokens
                            .push(Token::new_number(parse_result.unwrap(), i, slice_start..j, line));
                    } else {
                        line_result.tokens.push(Token::new_word(slice, i, slice_start..j, line));
                    }
                }
                slice_start = j + 1;
                continue;
            }

            // Check if characters are valid.
            if !(current_char.is_alphanumeric() || current_char == '.' || current_char == '_') {
                return Err(Error {
                    message: "Invalid character",
                    mark: Mark {
                        row: i,
                        column: j..j,
                        line
                    }
                });
            }
        }

        let line_length = line.len();

        // Check if unterminated string literal.
        if string_char != '\0' {
            return Err(Error {
                message: "unterminated string.",
                mark: Mark {
                    row: i,
                    column: slice_start..(line_length - 1),
                    line
                }
            });
        }

        //
        if slice_start != line_length {
            let slice = &line[slice_start..line_length];
            let parse_result = slice.parse::<f64>();
            if parse_result.is_ok() {
                line_result
                    .tokens
                    .push(Token::new_number(parse_result.unwrap(), i, slice_start..line_length, line));
            } else {
                line_result.tokens.push(Token::new_word(slice, i, slice_start..line_length, line));
            }
        }

        // Push `Line`.
        result.push(line_result);
    }

    Ok(result)
}
