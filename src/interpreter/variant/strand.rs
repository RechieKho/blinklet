use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use super::variant_ops::{VariantAdd, VariantSub, VariantMul, VariantDiv};
use std::sync::Arc;
use crate::mark::Mark;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Strand(String);

impl VariantAdd for Strand {
    fn add(&self, rhs: &Variant, _mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                Ok(Variant::STRAND(Strand::from(
                    self.0.clone() + rhs.represent()?.as_str(),
                )))
            }
        }
    }
}

impl VariantSub for Strand {
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

impl VariantMul for Strand {
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

impl VariantDiv for Strand {
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

impl Debug for Strand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Represent for Strand {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(self.0.clone())
    }
}

impl From<String> for Strand {
    fn from(value: String) -> Self {
        Strand(value)
    }
}

impl<'a> From<&'a str> for Strand {
    fn from(value: &'a str) -> Self {
        Strand(String::from(value))
    }
}

impl Into<String> for Strand {
    fn into(self) -> String {
        self.0
    }
}

impl Strand {
    pub fn as_str<'a>(&'a self) -> &'a str {
        self.0.as_str()
    }
}
