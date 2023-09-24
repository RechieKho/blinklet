use std::fmt::Display;
use std::ops::RangeInclusive;
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
    pub column: RangeInclusive<usize>,
}

impl MarkLine {
    pub fn new(name: Rc<String>, content: Rc<String>, row: usize) -> MarkLine {
        MarkLine { name, content, row }
    }
}

impl Mark {
    pub fn new(line: Rc<MarkLine>, column: RangeInclusive<usize>) -> Mark {
        Mark { line, column }
    }
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("📎 In code '{}':", self.line.name);
        let leader = format!("{:>5} |", self.line.row);
        let line = self.line.content.as_ref();
        let (start, end) = self.column.clone().into_inner();
        let underline = format!(
            "{:>width$}",
            "~".repeat(end - start + 1),
            width = leader.len() + line.len()
        );
        let rendering = format!("{header}\n{leader}{line}\n{underline}");
        f.write_str(&rendering)
    }
}
