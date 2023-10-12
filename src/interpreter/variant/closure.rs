use super::represent::Represent;
use super::table::Table;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use super::Variant;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::mark::Mark;
use crate::parser::atom::Atom;
use crate::raise_error;
use std::fmt::Debug;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Closure {
    pub mark: Mark,
    pub commands: Vec<Atom>,
    pub parent_scopes: Vec<Arc<Mutex<Table>>>,
}

impl VariantAdd for Closure {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be added with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantSub for Closure {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be subtracted with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantMul for Closure {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be multiplied with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantDiv for Closure {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be divided with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
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
            let value = context.resolve_variant(atom)?;
            slots.push(value);
        }

        let mut closure_context = Context::default();
        closure_context.slots = slots;
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        let result = closure_context.run_statements(&self.commands, Table::wrap_arc_mutex());
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        result
    }

    pub fn new(mark: Mark, commands: Vec<Atom>, parent_scopes: Vec<Arc<Mutex<Table>>>) -> Variant {
        let closure = Arc::new(Mutex::new(Closure {
            mark,
            commands,
            parent_scopes,
        }));
        Variant::CLOSURE(closure)
    }
}
