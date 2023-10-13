use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::table::Table;
use crate::parser::atom::Atom;

pub fn table_fn(context: &mut Context, _head: &Atom, body: &[Atom]) -> Result<Signal, Backtrace> {
    context.run_statements(body, Table::default())
}
