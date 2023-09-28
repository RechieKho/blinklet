use std::fmt::Display;
use std::ops::RangeInclusive;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MarkLine {
    pub name: Arc<String>,
    pub content: Arc<String>,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub struct Mark {
    pub line: Arc<MarkLine>,
    pub column: RangeInclusive<usize>,
}

impl MarkLine {
    pub fn new(name: Arc<String>, content: Arc<String>, row: usize) -> MarkLine {
        MarkLine { name, content, row }
    }
}

impl Mark {
    pub fn new(line: Arc<MarkLine>, column: RangeInclusive<usize>) -> Mark {
        Mark { line, column }
    }
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("In code '{}':", self.line.name);
        let leader = format!("{:>5} |", self.line.row);
        let line = self.line.content.as_ref();
        let (start, end) = self.column.clone().into_inner();
        let underline = format!(
            "{:>width$}",
            "~".repeat(end - start),
            width = leader.len() + end
        );
        let rendering = format!("{header}\n{leader}{line}\n{underline}");
        f.write_str(&rendering)
    }
}
