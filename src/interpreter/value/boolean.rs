use super::{represent::Represent, Value};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone, Copy)]
pub struct Boolean(bool);

impl<T> Add<T> for Boolean {
    type Output = Result<Value, Backtrace>;

    fn add(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Boolean cannot be added.");
    }
}

impl<T> Sub<T> for Boolean {
    type Output = Result<Value, Backtrace>;

    fn sub(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Boolean cannot be subtracted.");
    }
}

impl<T> Mul<T> for Boolean {
    type Output = Result<Value, Backtrace>;

    fn mul(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Boolean cannot be multiplied.");
    }
}

impl<T> Div<T> for Boolean {
    type Output = Result<Value, Backtrace>;

    fn div(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Boolean cannot be divided.");
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
