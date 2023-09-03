use std::vec::Vec;
use std::string::String;
use std::result::Result;
use super::error::ParserError;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    WORD(&'a str, (usize, usize)),
    STRING(&'a str, (usize, usize)),
    NUMBER(f64, (usize, usize))
}

#[derive(Debug, Default)]
pub struct Line<'a> {
    pub tokens : Vec<Token<'a>>,
    pub indent_count : usize,
    pub row : usize
}

pub fn lex<'a>(code: &'a String) -> Result<Vec<Line<'a>>, ParserError> {
    let mut result : Vec<Line<'a>> = Vec::new();
    let mut indent_char = '\0';
    let mut indent_factor = 0usize;
    for (i, line) in code.lines().enumerate() {
        if line.len() == 0 {continue;}

        let mut line_result : Line<'a> = Line::default();
        let mut is_indent_scanned = false;
        let mut string_char = '\0';
        let mut slice_start = 0usize;

        line_result.row = i;

        for (j, current_char) in line.chars().into_iter().enumerate() {
            // Collect indentation count.
            if !is_indent_scanned {
                if current_char.is_whitespace() {
                    if indent_char == '\0' { indent_char = current_char; }
                    if current_char != indent_char {
                        return Err(
                            ParserError {
                                message: "Inconsistent indentation character.",
                                position: (i, j)
                            }
                        );
                    }
                    line_result.indent_count += 1;
                    continue;
                } else {
                    is_indent_scanned = true;
                    slice_start = j;
                    if indent_factor == 0 { indent_factor = line_result.indent_count; }
                    if indent_factor != 0 { // We are not using else to consider the value change.
                        if line_result.indent_count % indent_factor != 0 {
                            return Err(
                                ParserError {
                                    message: "Inconsistent indentation factor.",
                                    position: (i, j)
                                }
                            );
                        }
                        line_result.indent_count /= indent_factor
                    }
                }
            }

            // Check if it is in string literal
            if string_char != '\0' {
                if current_char == string_char {
                    line_result.tokens.push(Token::STRING(&line[slice_start..j], (i, j)));
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
                    if parse_result.is_ok() { line_result.tokens.push(Token::NUMBER(parse_result.unwrap(), (i, j))); } 
                    else { line_result.tokens.push(Token::WORD(slice, (i, j))); }
                }
                slice_start = j + 1;
                continue;
            }

            // Check if it is a whitespace.
            if current_char.is_whitespace() {
                if slice_start != j { 
                    let slice = &line[slice_start..j];
                    let parse_result = slice.parse::<f64>();
                    if parse_result.is_ok() { line_result.tokens.push(Token::NUMBER(parse_result.unwrap(), (i, j))); } 
                    else { line_result.tokens.push(Token::WORD(slice, (i, j))); }
                }
                slice_start = j + 1;
                continue;
            }

            // Check if characters are valid.
            if !(
                current_char.is_alphanumeric() || 
                current_char == '.' ||
                current_char == '_'
                ) {
                return Err(
                    ParserError { 
                        message: "Invalid character",
                        position: (i, j)
                    }
                );
            }
        }

        let line_length = line.len();

        // Check if unterminated string literal.
        if string_char != '\0' {
            return Err(
                ParserError { 
                    message: "unterminated string.",
                    position: (i, line_length - 1)
                }
            );
        }

        // 
        if slice_start != line_length { 
            let slice = &line[slice_start..line_length];
            let parse_result = slice.parse::<f64>();
            if parse_result.is_ok() { line_result.tokens.push(Token::NUMBER(parse_result.unwrap(), (i, slice_start))); } 
            else { line_result.tokens.push(Token::WORD(slice, (i, slice_start))); }
        }

        // Push `Line`.
        result.push(line_result);
    }
    
    Ok(result)
}
