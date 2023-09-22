use super::mark::Mark;
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub mark: Option<Rc<Mark>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("Error: {}", self.message);
        if self.mark.is_none() {
            f.pad(&header)
        } else {
            let mark = self.mark.clone().unwrap();
            let rendering = format!("{header}\n{:5}\n", mark);
            f.pad(&rendering)
        }
    }
}
