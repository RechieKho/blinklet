use super::mark::Mark;

#[derive(Debug, Clone)]
pub struct Error<'name, 'code> {
    pub message: String,
    pub mark: Option<Mark<'name, 'code>>,
}
