use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use super::variant_ops::{VariantAdd, VariantSub, VariantMul, VariantDiv};
use std::sync::Arc;
use crate::mark::Mark;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Null();

impl VariantAdd for Null {
    fn add(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
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

impl VariantSub for Null {
    fn sub(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
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

impl VariantMul for Null {
    fn mul(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
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

impl VariantDiv for Null {
    fn div(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
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

impl Debug for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl Represent for Null {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(String::from("null"))
    }
}
