use crate::log::Log;
use std::fmt::{Debug, Display};

#[macro_export]
macro_rules! raise_error {
    ($mark:expr, $($message:expr),*) => {
        return Err(crate::backtrace::Backtrace::new(crate::log::Log::error(format!($($message),*), $mark)));
    };
}

#[macro_export]
macro_rules! raise_bug {
    ($mark:expr, $($message:expr),*) => {
        return Err(crate::backtrace::Backtrace::new(crate::log::Log::bug(format!($($message),*), $mark)));
    };
}

#[derive(Debug, Clone)]
pub struct Backtrace(Vec<Log>);

impl Backtrace {
    pub fn new(log: Log) -> Backtrace {
        Backtrace(vec![log])
    }

    pub fn push(&mut self, log: Log) {
        self.0.push(log);
    }
}

impl Display for Backtrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for log in self.0.iter().rev() {
            let rendering = format!("{log}\n");
            f.write_str(&rendering)?;
        }
        Ok(())
    }
}
