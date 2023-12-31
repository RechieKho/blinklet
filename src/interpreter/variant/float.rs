use super::strand::Strand;
use super::variant_ops::{
    VariantAdd, VariantDiv, VariantDuplicate, VariantEq, VariantG, VariantGe, VariantL, VariantLe,
    VariantMul, VariantSub,
};
use super::{represent::Represent, Variant};
use crate::interpreter::context::Context;
use crate::mark::Mark;
use crate::{backtrace::Backtrace, raise_error};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Float(f64);

impl VariantAdd for Float {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 + rhs_float)))
            }
            Variant::STRAND(strand) => {
                let mut self_string = self.represent(mark)?;
                let mut rhs_string: String = strand.clone().into();
                self_string.push_str(&mut rhs_string);
                Ok(Variant::STRAND(Strand::from(self_string)))
            }
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

impl VariantSub for Float {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 - rhs_float)))
            }
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

impl VariantMul for Float {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 * rhs_float)))
            }
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

impl VariantDiv for Float {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 / rhs_float)))
            }
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

impl VariantEq for Float {
    fn eq(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => Ok(self.0 == float.0),
            _ => Ok(false),
        }
    }
}

impl VariantGe for Float {
    fn ge(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => Ok(self.0 >= float.0),
            _ => Ok(false),
        }
    }
}

impl VariantG for Float {
    fn g(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => Ok(self.0 > float.0),
            _ => Ok(false),
        }
    }
}

impl VariantLe for Float {
    fn le(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => Ok(self.0 <= float.0),
            _ => Ok(false),
        }
    }
}

impl VariantL for Float {
    fn l(&self, rhs: &Variant, _mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => Ok(self.0 < float.0),
            _ => Ok(false),
        }
    }
}

impl VariantDuplicate for Float {
    fn duplicate(&self, _mark: Option<Mark>, _context: &mut Context) -> Result<Variant, Backtrace> {
        Ok(Variant::FLOAT(self.clone()))
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Represent for Float {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(format!("{}", self.0))
    }
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        self.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value)
    }
}
