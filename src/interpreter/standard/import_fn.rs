use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::resource::ResourcePath;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count, atom_as_identifier};

pub fn import_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, 1);
    let identifier = atom_as_identifier!(&body[0]);
    context.run_resource(ResourcePath::try_from(identifier.clone())?)
}
