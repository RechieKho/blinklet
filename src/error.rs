use super::mark::Mark;

#[derive(Debug, Clone)]
pub struct Error<'code> {
    pub message: String,
    pub mark: Option<Mark<'code>>,
}
