use super::mark::Mark;

#[derive(Debug)]
pub struct Error<'code> {
    pub message: String,
    pub mark: Option<Mark<'code>>,
}
