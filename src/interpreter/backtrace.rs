use crate::{error::Error, mark::Mark};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Backtrace(Vec<Error>);

impl Backtrace {
    pub fn new(error: Error) -> Backtrace {
        Backtrace(vec![error])
    }

    pub fn push(&mut self, error: Error) {
        self.0.push(error);
    }

    pub fn trace<T>(
        result: Result<T, Backtrace>,
        mark: &Option<Rc<Mark>>,
    ) -> Result<T, Backtrace>
    where
        T: Debug,
    {
        if result.is_ok() || mark.is_none() {
            return result;
        }

        let mut backtrace = result.unwrap_err();
        backtrace.push(Error {
            message: format!("Trace."),
            mark: mark.clone(),
        });
        Err(backtrace)
    }
}

impl Display for Backtrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in self.0.iter() {
            let rendering = format!("{error}");
            f.pad(&rendering)?;
        }
        Ok(())
    }
}
