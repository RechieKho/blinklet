use super::object::Object;
use crate::parser::command::Atom;

pub fn greet<'code>(_context: &mut Object<'code>, _body: &[Atom<'code>]) {
    println!("Hello world");
}
