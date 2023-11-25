use super::variant_ops::{
    VariantAdd, VariantDiv, VariantDuplicate, VariantEq, VariantG, VariantGe, VariantL, VariantLe,
    VariantMul, VariantSub,
};
use super::{represent::Represent, Variant};
use crate::mark::Mark;
use crate::{backtrace::Backtrace, raise_error};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Boolean(bool);

impl VariantAdd for Boolean {
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

impl VariantSub for Boolean {
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

impl VariantMul for Boolean {
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

impl VariantDiv for Boolean {
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

impl VariantEq for Boolean {
    fn eq(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::BOOL(boolean) => Ok(self.0 == boolean.0),
            _ => Ok(false),
        }
    }
}

impl VariantGe for Boolean {
    fn ge(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::BOOL(boolean) => Ok(self.0 == boolean.0),
            _ => Ok(false),
        }
    }
}

impl VariantG for Boolean {
    fn g(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantLe for Boolean {
    fn le(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::BOOL(boolean) => Ok(self.0 == boolean.0),
            _ => Ok(false),
        }
    }
}

impl VariantL for Boolean {
    fn l(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            _ => Ok(false),
        }
    }
}

impl VariantDuplicate for Boolean {
    fn duplicate(&self, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        Ok(Variant::BOOL(self.clone()))
    }
}

impl Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Represent for Boolean {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(String::from(if self.is_true() { "true" } else { "false" }))
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.0
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

impl Boolean {
    pub fn is_true(&self) -> bool {
        self.0
    }
}
