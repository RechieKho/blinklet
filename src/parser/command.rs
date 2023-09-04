use super::error::ParserError;
use super::lexer::Line;
use super::lexer::Token;

pub type Command<'a> = Vec<Atom<'a>>;

#[derive(Debug)]
pub enum Atom<'a> {
    WORD(&'a str, (usize, usize)),
    STRING(&'a str, (usize, usize)),
    NUMBER(f64, (usize, usize)),
    COMMAND(Command<'a>)
}


pub fn generate_commands<'a>(lot: &Vec<Line<'a>>) -> Result<Vec<Command<'a>>, ParserError> {
    let mut result : Vec<Command<'a>> = Vec::new();
    let mut current_indent_count = 0usize;

    fn get_subcommand_mut<'a, 'b>(command: &'a mut Command<'b>, nesting: usize) -> Option<&'a mut Command<'b>> {
        let mut subcommand = command;
        for _ in 0..nesting {
            let last = subcommand.last_mut();
            if last.is_none() { return None; }
            let atom = last.unwrap();
            if let Atom::COMMAND(c) = atom { subcommand = c; }
            else { return None; }
        }
        Some(subcommand)
    }

    for line in lot.iter() {
        let indent_displacement = line.indent_count as isize - current_indent_count as isize;
        if indent_displacement > 1 {
            return Err(ParserError { message: "Excessive indentation.", position: (line.row, 0) })
        }

        let mut atoms : Vec<Atom<'a>> = Vec::default();
        for token in line.tokens.iter() {
            // Collect atoms.
            let new_atom : Atom<'a> = match token {
                Token::WORD(d, p) => Atom::WORD(*d, *p),
                Token::STRING(d, p) => Atom::STRING(d, *p),
                Token::NUMBER(d, p) => Atom::NUMBER(*d, *p)
            };
            atoms.push(new_atom);
        }

        if atoms.len() == 0 { 
            current_indent_count = line.indent_count; 
            continue; 
        }

        // Indentation at the very first command, this is a sin.
        if result.len() == 0 && line.indent_count != 0 {
            return Err(ParserError { message: "Unexpected indentation.", position: (line.row, 0) });
        }

        // Just append to the result since there is no indentation.
        if line.indent_count == 0 {
            result.push(atoms);
            current_indent_count = line.indent_count; 
            continue;
        }


        // There is indentation, get the parent command and push the subcommand.
        let parent_command = get_subcommand_mut(result.last_mut().unwrap(), line.indent_count - 1).unwrap();
        match atoms.first().unwrap() {
            Atom::WORD(d, _) => if *d == "ensuing" {
                    parent_command.append(&mut atoms); 
                    current_indent_count = line.indent_count; 
                    continue;
            }
            Atom::STRING(_, p) => return Err(
                ParserError { 
                    message: "String as the head of a command is forbidden.", 
                    position: *p
                }
            ),
            Atom::NUMBER(_, p) => return Err(
                ParserError { 
                    message: "Number as the head of a command is forbidden.", 
                    position: *p
                }
            ),
            _ => unreachable!("Commandas the head of a command should be unreachable.")
        }
        parent_command.push(Atom::COMMAND(atoms));
        current_indent_count = line.indent_count; 
    }

    Ok(result)
}
