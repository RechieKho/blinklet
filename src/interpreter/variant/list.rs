use super::null::Null;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use super::{represent::Represent, Variant};
use crate::mark::Mark;
use crate::mutex_lock_unwrap;
use crate::{backtrace::Backtrace, raise_error};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct List(Arc<Mutex<Vec<Variant>>>);

impl VariantAdd for List {
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

impl VariantSub for List {
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

impl VariantMul for List {
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

impl VariantDiv for List {
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

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("list")
    }
}

impl Represent for List {
    fn represent(&self, mark: Option<Mark>) -> Result<String, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark.clone());
        let representations = guard
            .iter()
            .map(|x| match x {
                Variant::STRAND(strand) => Ok(format!("\"{}\"", strand.as_str())),
                _ => x.represent(mark.clone()),
            })
            .collect::<Result<Vec<String>, Backtrace>>()?;
        Ok(format!("[{}]", representations.join(", ")))
    }
}

impl From<Vec<Variant>> for List {
    fn from(value: Vec<Variant>) -> Self {
        List(Arc::new(Mutex::new(value)))
    }
}

impl List {
    pub fn push(&mut self, variant: Variant, mark: Option<Mark>) -> Result<(), Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        guard.push(variant);
        Ok(())
    }

    pub fn pop(&mut self, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        let variant = guard.pop();
        Ok(if variant.is_none() {
            Variant::NULL(Null::new())
        } else {
            variant.unwrap()
        })
    }
}
