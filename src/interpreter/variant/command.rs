use super::represent::Represent;
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
use std::sync::Arc;

#[derive(Clone)]
pub struct Command {
    callable: Arc<dyn Fn(&mut Context, &Atom, &[Atom]) -> Result<Signal, Backtrace>>,
}

unsafe impl Sync for Command {}
unsafe impl Send for Command {}

impl VariantAdd for Command {
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

impl VariantSub for Command {
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

impl VariantMul for Command {
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

impl VariantDiv for Command {
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

impl VariantEq for Command {
    fn eq(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::COMMAND(command) => Ok(Arc::ptr_eq(&self.callable, &command.callable)),
            _ => Ok(false),
        }
    }
}

impl VariantGe for Command {
    fn ge(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::COMMAND(command) => Ok(Arc::ptr_eq(&self.callable, &command.callable)),
            _ => Ok(false),
        }
    }
}

impl VariantG for Command {
    fn g(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantLe for Command {
    fn le(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::COMMAND(command) => Ok(Arc::ptr_eq(&self.callable, &command.callable)),
            _ => Ok(false),
        }
    }
}

impl VariantL for Command {
    fn l(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantDuplicate for Command {
    fn duplicate(&self, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        Ok(Variant::COMMAND(self.clone()))
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<Command>")
    }
}

impl Represent for Command {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(String::from("<Command>"))
    }
}

impl Command {
    pub fn new<T>(callable: T) -> Self
    where
        T: Fn(&mut Context, &Atom, &[Atom]) -> Result<Signal, Backtrace> + 'static,
    {
        Command {
            callable: Arc::new(callable),
        }
    }

    pub fn call(
        &self,
        context: &mut Context,
        head: &Atom,
        atoms: &[Atom],
    ) -> Result<Signal, Backtrace> {
        (self.callable)(context, head, atoms)
    }
}
