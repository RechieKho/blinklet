use crate::{backtrace::Backtrace, mark::Mark};

pub trait Represent {
    fn represent(&self, mark: Option<Mark>) -> Result<String, Backtrace>;
}
