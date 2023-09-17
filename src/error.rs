use super::mark::Mark;

#[derive(Debug)]
pub struct Error<'code> {
    pub message: &'static str,
    pub mark: Mark<'code>,
}
