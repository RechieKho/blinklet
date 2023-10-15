use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::resource::ResourcePath;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;
use crate::{assert_atoms_count, atom_as_identifier};

pub fn import_fn(context: &mut Context, head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count!(body, Some(head.mark.clone()), 1);
    let identifier = atom_as_identifier!(&body[0]);
    context.run_code(ResourcePath::try_from(identifier.clone())?)
}
