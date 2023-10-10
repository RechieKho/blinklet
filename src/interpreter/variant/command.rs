use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::parser::atom::Atom;
use std::fmt::Debug;
use std::sync::Arc;

use super::represent::Represent;

pub struct Command {
    callable: Box<dyn Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace>>,
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("command") // TODO
    }
}

impl Represent for Command {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(String::from("command")) // TODO
    }
}

impl Command {
    pub fn wrap_arc<T>(callable: T) -> Arc<Self>
    where
        T: Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace> + 'static,
    {
        Arc::new(Command {
            callable: Box::new(callable),
        })
    }

    pub fn call(&self, context: &mut Context, atoms: &[Atom]) -> Result<Signal, Backtrace> {
        (self.callable)(context, atoms)
    }
}
