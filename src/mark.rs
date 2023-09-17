use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Mark<'code> {
    pub row: usize,
    pub column: Range<usize>,
    pub line: &'code str
}
