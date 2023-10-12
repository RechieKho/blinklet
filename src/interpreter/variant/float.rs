use super::strand::Strand;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use super::{represent::Represent, Variant};
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
                let mut self_string = self.represent()?;
                let mut rhs_string: String = strand.clone().into();
                self_string.push_str(&mut rhs_string);
                Ok(Variant::STRAND(Strand::from(self_string)))
            }
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

impl VariantSub for Float {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 - rhs_float)))
            }
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

impl VariantMul for Float {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 * rhs_float)))
            }
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

impl VariantDiv for Float {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            Variant::FLOAT(float) => {
                let rhs_float: f64 = float.clone().into();
                Ok(Variant::FLOAT(Float::from(self.0 / rhs_float)))
            }
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

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Represent for Float {
    fn represent(&self) -> Result<String, Backtrace> {
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

impl Float {
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
}
