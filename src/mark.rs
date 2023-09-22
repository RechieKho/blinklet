use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MarkLine {
    pub name: Rc<String>,
    pub content: Rc<String>,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub struct Mark {
    pub line: Rc<MarkLine>,
    pub column: Range<usize>,
}

impl MarkLine {
    pub fn new(name: Rc<String>, content: Rc<String>, row: usize) -> MarkLine {
        MarkLine { name, content, row }
    }
}

impl Mark {
    pub fn new(line: Rc<MarkLine>, column: Range<usize>) -> Mark {
        Mark { line, column }
    }
}
