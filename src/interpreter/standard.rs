use super::object::Object;
use crate::error::Error;
use crate::parser::command::Atom;

pub fn greet<'code>(
    _context: &mut Object<'code>,
    _body: &[Atom<'code>],
) -> Result<(), Error<'code>> {
    println!("Hello world");
    Ok(())
}
