use crate::handler::Statement;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    IMP,
    IO,
    StackManip,
    Arithmetic,
    FlowCtrl,
    HeapAccess,
    Unknown(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse > {:?}", &self)
    }
}

impl std::error::Error for ParseError {}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Parser {
    fn next_statement(&mut self) -> ParseResult<Statement>;
}
