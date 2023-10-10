use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone, Copy)]
pub struct Null();

impl Add<Variant> for Null {
    type Output = Result<Variant, Backtrace>;

    fn add(self, rhs: Variant) -> Self::Output {
        raise_error!(
            None,
            "`{}` cannot be added with `{}`.",
            self.represent()?,
            rhs.represent()?
        );
    }
}

impl Sub<Variant> for Null {
    type Output = Result<Variant, Backtrace>;

    fn sub(self, rhs: Variant) -> Self::Output {
        raise_error!(
            None,
            "`{}` cannot be subtracted with `{}`.",
            self.represent()?,
            rhs.represent()?
        );
    }
}

impl Mul<Variant> for Null {
    type Output = Result<Variant, Backtrace>;

    fn mul(self, rhs: Variant) -> Self::Output {
        raise_error!(
            None,
            "`{}` cannot be multiplied with `{}`.",
            self.represent()?,
            rhs.represent()?
        );
    }
}

impl Div<Variant> for Null {
    type Output = Result<Variant, Backtrace>;

    fn div(self, rhs: Variant) -> Self::Output {
        raise_error!(
            None,
            "`{}` cannot be divided with `{}`.",
            self.represent()?,
            rhs.represent()?
        );
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
