use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone, Copy)]
pub struct Boolean(bool);

impl Add<Variant> for Boolean {
    type Output = Result<Variant, Backtrace>;

    fn add(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be added with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Sub<Variant> for Boolean {
    type Output = Result<Variant, Backtrace>;

    fn sub(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be subtracted with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Mul<Variant> for Boolean {
    type Output = Result<Variant, Backtrace>;

    fn mul(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be multiplied with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Div<Variant> for Boolean {
    type Output = Result<Variant, Backtrace>;

    fn div(self, rhs: Variant) -> Self::Output {
        raise_error!(None, "`{}` cannot be divided with `{}`.", self.represent()?, rhs.represent()?);
    }
}

impl Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Represent for Boolean {
    fn represent(&self) -> Result<String, Backtrace> {
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
