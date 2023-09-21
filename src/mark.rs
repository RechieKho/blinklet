use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Mark<'name, 'code> {
    pub name: &'name String,
    pub row: usize,
    pub column: Range<usize>,
    pub line: &'code str,
}
