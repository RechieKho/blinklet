use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone)]
pub struct Strand(String);

impl Add<Variant> for Strand {
    type Output = Result<Variant, Backtrace>;

    fn add(self, rhs: Variant) -> Self::Output {
        Ok(Variant::STRAND(Strand::from(self.0 + rhs.represent()?.as_str())))
    }
}

impl Sub<Variant> for Strand {
    type Output = Result<Variant, Backtrace>;

    fn sub(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be subtracted with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Mul<Variant> for Strand {
    type Output = Result<Variant, Backtrace>;

    fn mul(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be multiplied with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Div<Variant> for Strand {
    type Output = Result<Variant, Backtrace>;

    fn div(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be divided with `{}`.", self.represent()?, rhs.represent()?);
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
