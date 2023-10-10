use super::{represent::Represent, Value};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone, Copy)]
pub struct Null();

impl<T> Add<T> for Null {
    type Output = Result<Value, Backtrace>;

    fn add(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Null cannot be added.");
    }
}

impl<T> Sub<T> for Null {
    type Output = Result<Value, Backtrace>;

    fn sub(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Null cannot be subtracted.");
    }
}

impl<T> Mul<T> for Null {
    type Output = Result<Value, Backtrace>;

    fn mul(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Null cannot be multiplied.");
    }
}

impl<T> Div<T> for Null {
    type Output = Result<Value, Backtrace>;

    fn div(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Null cannot be divided.");
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
