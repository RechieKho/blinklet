use super::represent::Represent;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use super::Variant;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::mark::Mark;
use crate::parser::atom::Atom;
use crate::raise_error;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct Command {
    callable: Arc<dyn Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace>>,
}

impl VariantAdd for Command {
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

impl VariantSub for Command {
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

impl VariantMul for Command {
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

impl VariantDiv for Command {
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
    pub fn new<T>(callable: T) -> Self
    where
        T: Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace> + 'static,
    {
        Command {
            callable: Arc::new(callable),
        }
    }

    pub fn call(&self, context: &mut Context, atoms: &[Atom]) -> Result<Signal, Backtrace> {
        (self.callable)(context, atoms)
    }
}
