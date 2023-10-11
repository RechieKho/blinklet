use crate::backtrace::Backtrace;
use std::sync::Arc;
use crate::mark::Mark;
use super::Variant;

pub trait VariantAdd {
    fn add(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace>;
}

pub trait VariantSub {
    fn sub(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace>;
}

pub trait VariantMul {
    fn mul(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace>;
}

pub trait VariantDiv {
    fn div(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace>;
}