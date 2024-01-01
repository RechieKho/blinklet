use super::represent::Represent;
use super::table::Table;
use super::variant_ops::{
    VariantAdd, VariantDiv, VariantDuplicate, VariantEq, VariantG, VariantGe, VariantL, VariantLe,
    VariantMul, VariantSub,
};
use super::Variant;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::mark::Mark;
use crate::parser::atom::Atom;
use crate::raise_error;
use std::fmt::Debug;
use std::mem;

#[derive(Clone)]
pub struct Closure {
    pub mark: Mark,
    pub commands: Vec<Atom>,
    pub parent_scopes: Vec<Table>,
}

impl VariantAdd for Closure {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be added with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
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
                    mark.clone(),
                    "`{}` cannot be subtracted with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
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
                    mark.clone(),
                    "`{}` cannot be multiplied with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
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
                    mark.clone(),
                    "`{}` cannot be divided with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantEq for Closure {
    fn eq(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::CLOSURE(closure) => self.is_closure_eq(closure, mark),
            _ => Ok(false),
        }
    }
}

impl VariantGe for Closure {
    fn ge(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::CLOSURE(closure) => self.is_closure_eq(closure, mark),
            _ => Ok(false),
        }
    }
}

impl VariantG for Closure {
    fn g(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantLe for Closure {
    fn le(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::CLOSURE(closure) => self.is_closure_eq(closure, mark),
            _ => Ok(false),
        }
    }
}

impl VariantL for Closure {
    fn l(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantDuplicate for Closure {
    fn duplicate(&self, _mark: Option<Mark>, context: &mut Context) -> Result<Variant, Backtrace> {
        let new_closure = Closure::new(
            self.mark.clone(),
            self.commands.clone(),
            context.scopes.clone(),
        );
        Ok(Variant::CLOSURE(new_closure))
    }
}

impl Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<Closure at row {}, in '{}'>",
            self.mark.line.row, self.mark.line.name
        ))
    }
}

impl Represent for Closure {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(format!(
            "<Closure at row {}, in '{}'>",
            self.mark.line.row, self.mark.line.name
        ))
    }
}

impl Closure {
    pub fn call_mut(&mut self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        let mut slots: Vec<Variant> = Vec::new();
        for atom in body.iter() {
            let value = context.resolve_variant(atom)?;
            slots.push(value);
        }
        let mut closure_context = Context::new()?;
        closure_context.slots = slots;
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes); // Install parent scopes into the context.
        let result = closure_context.run_statements(&self.commands, Table::default());
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes); // Retrieve parent scopes back.
        result
    }

    pub fn new(mark: Mark, commands: Vec<Atom>, parent_scopes: Vec<Table>) -> Self {
        Closure {
            mark,
            commands,
            parent_scopes,
        }
    }

    pub fn is_closure_eq(&self, other: &Self, mark: Option<Mark>) -> Result<bool, Backtrace> {
        // Comparing parent scope.
        if self.parent_scopes.len() != other.parent_scopes.len() {
            return Ok(false);
        }

        for i in 0..self.parent_scopes.len() {
            let self_element = self.parent_scopes.get(i).unwrap();
            let other_element = self.parent_scopes.get(i).unwrap();
            if !self_element.is_table_eq(other_element, mark.clone())? {
                return Ok(false);
            }
        }

        Ok(self.mark == other.mark)
    }
}
