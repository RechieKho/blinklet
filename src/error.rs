use super::mark::Mark;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub mark: Option<Rc<Mark>>,
}
