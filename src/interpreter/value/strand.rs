use super::{represent::Represent, Value};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone)]
pub struct Strand(String);

impl<T> Add<T> for Strand {
    type Output = Result<Value, Backtrace>;

    fn add(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Strand cannot be added.");
    }
}

impl<T> Sub<T> for Strand {
    type Output = Result<Value, Backtrace>;

    fn sub(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Strand cannot be subtracted.");
    }
}

impl<T> Mul<T> for Strand {
    type Output = Result<Value, Backtrace>;

    fn mul(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Strand cannot be multiplied.");
    }
}

impl<T> Div<T> for Strand {
    type Output = Result<Value, Backtrace>;

    fn div(self, _rhs: T) -> Self::Output {
        raise_error!(None, "Strand cannot be divided.");
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
