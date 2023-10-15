use crate::assert_atoms_count_min;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::table::Table;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn when_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 1);
    let boolean = context.resolve_boolean(&body[0])?;
    if boolean.into() {
        context.run_statements(&body[1..], Table::default())
    } else {
        Ok(Signal::COMPLETE(Variant::NULL(Null::new())))
    }
}
