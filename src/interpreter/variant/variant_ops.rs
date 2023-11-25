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

pub trait VariantEq {
    fn eq(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace>;
}

pub trait VariantGe {
    fn ge(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace>;
}

pub trait VariantG {
    fn g(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace>;
}

pub trait VariantLe {
    fn le(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace>;
}

pub trait VariantL {
    fn l(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace>;
}

pub trait VariantDuplicate {
    fn duplicate(&self, mark: Option<Mark>) -> Result<Variant, Backtrace>;
}
