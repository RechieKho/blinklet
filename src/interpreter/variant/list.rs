use super::{represent::Represent, Variant};
use crate::{backtrace::Backtrace, raise_error};
use std::{fmt::Debug, ops::Add, ops::Div, ops::Mul, ops::Sub};

#[derive(Clone)]
pub struct List(Vec<Variant>);

impl Add<Variant> for List {
    type Output = Result<Variant, Backtrace>;

    fn add(mut self, rhs: Variant) -> Self::Output {
        match rhs {
            Variant::LIST(list) => {
                self.0.append(&mut list.into());
                Ok(Variant::LIST(self))
            }
            _ => {
                raise_error!(
                    None,
                    "`{}` cannot be added with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl Sub<Variant> for List {
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

impl Mul<Variant> for List {
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

impl Div<Variant> for List {
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

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("list")
    }
}

impl Represent for List {
    fn represent(&self) -> Result<String, Backtrace> {
        let representations = self
            .0
            .iter()
            .map(|x| match x {
                Variant::STRAND(strand) => Ok(format!("\"{}\"", strand.as_str())),
                _ => x.represent(),
            })
            .collect::<Result<Vec<String>, Backtrace>>()?;
        Ok(format!("[{}]", representations.join(", ")))
    }
}

impl Into<Vec<Variant>> for List {
    fn into(self) -> Vec<Variant> {
        self.0
    }
}

impl From<Vec<Variant>> for List {
    fn from(value: Vec<Variant>) -> Self {
        List(value)
    }
}
