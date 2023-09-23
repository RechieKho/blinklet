use crate::{log::Log, mark::Mark};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Backtrace(Vec<Log>);

impl Backtrace {
    pub fn new(log: Log) -> Backtrace {
        Backtrace(vec![log])
    }

    pub fn push(&mut self, log: Log) {
        self.0.push(log);
    }

    pub fn trace<T>(result: Result<T, Backtrace>, mark: &Option<Rc<Mark>>) -> Result<T, Backtrace>
    where
        T: Debug,
    {
        if result.is_ok() || mark.is_none() {
            return result;
        }

        let mut backtrace = result.unwrap_err();
        backtrace.push(Log::trace(mark.clone()));
        Err(backtrace)
    }
}

impl Display for Backtrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for log in self.0.iter() {
            let rendering = format!("{log}");
            f.pad(&rendering)?;
        }
        Ok(())
    }
}
