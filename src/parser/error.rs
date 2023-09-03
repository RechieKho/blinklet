#[derive(Debug)]
pub struct ParserError {
    pub message: &'static str,
    pub position : (usize, usize) // Row (line) and column.
}
