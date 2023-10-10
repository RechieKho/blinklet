use crate::backtrace::Backtrace;

pub trait Represent {
    fn represent(&self) -> Result<String, Backtrace>;
}
