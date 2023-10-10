use super::represent::Represent;
use super::scope::Scope;
use super::table::Table;
use super::Variant;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::mark::Mark;
use crate::parser::atom::Atom;
use std::fmt::Debug;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Closure {
    pub mark: Arc<Mark>,
    pub commands: Vec<Atom>,
    pub parent_scopes: Vec<Arc<Mutex<dyn Table>>>,
}

impl Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("closure") // TODO
    }
}

impl Represent for Closure {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(String::from("closure")) // TODO
    }
}

impl Closure {
    pub fn call_mut(&mut self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        let mut slots: Vec<Variant> = Vec::new();
        for atom in body.iter().skip(1) {
            let value = context.resolve_value(atom)?;
            slots.push(value);
        }

        let mut closure_context = Context::default();
        closure_context.slots = slots;
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        let result = closure_context.run_statements(&self.commands, Scope::wrap_arc_mutex());
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        result
    }

    pub fn new(
        mark: Arc<Mark>,
        commands: Vec<Atom>,
        parent_scopes: Vec<Arc<Mutex<dyn Table>>>,
    ) -> Variant {
        let closure = Arc::new(Mutex::new(Closure {
            mark,
            commands,
            parent_scopes,
        }));
        Variant::CLOSURE(closure)
    }
}
