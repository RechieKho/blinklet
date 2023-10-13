use super::boolean::Boolean;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub, VariantEq, VariantGe, VariantG, VariantLe, VariantL};
use super::{represent::Represent, Variant};
use crate::mark::Mark;
use crate::{backtrace::Backtrace, raise_error};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Null();

impl VariantAdd for Null {
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

impl VariantSub for Null {
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

impl VariantMul for Null {
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

impl VariantDiv for Null {
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

impl VariantEq for Null {
    fn eq(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::NULL(_) => Ok(Variant::BOOL(Boolean::from(true))),
            _ => Ok(Variant::BOOL(Boolean::from(false)))
        }
    }
}

impl VariantGe for Null {
    fn ge(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::NULL(_) => Ok(Variant::BOOL(Boolean::from(true))),
            _ => Ok(Variant::BOOL(Boolean::from(false)))
        }
    }
}

impl VariantG for Null {
    fn g(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => Ok(Variant::BOOL(Boolean::from(false)))
        }
    }
}

impl VariantLe for Null {
    fn le(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::NULL(_) => Ok(Variant::BOOL(Boolean::from(true))),
            _ => Ok(Variant::BOOL(Boolean::from(false)))
        }
    }
}

impl VariantL for Null {
    fn l(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => Ok(Variant::BOOL(Boolean::from(false)))
        }
    }
}

impl Debug for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl Represent for Null {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(String::from("null"))
    }
}

impl Null {
    pub fn new() -> Null {
        Null()
    }
}
