use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::table::Table;
use crate::parser::atom::Atom;

pub fn table_fn(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    let table = Table::default();
    context.run_statements(&body[1..], table)
}
