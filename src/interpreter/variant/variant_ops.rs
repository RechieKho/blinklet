use super::Variant;
use crate::backtrace::Backtrace;
use crate::mark::Mark;

pub trait VariantAdd {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace>;
}

pub trait VariantSub {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace>;
}

pub trait VariantMul {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace>;
}

pub trait VariantDiv {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace>;
}
