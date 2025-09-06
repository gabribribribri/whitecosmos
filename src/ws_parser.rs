
pub enum WSStatement {
    PopStackOutputNumber
}

use std::io;

#[derive(Debug)]
pub enum WSParseError {
    EOF,
    IMP,
    IO,
    StackManip,
    Arithmetic,
    FlowCtrl,
    HeapAccess,
    Unknown(String),
}

impl std::fmt::Display for WSParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for WSParseError {}

impl From<io::Error> for WSParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::EOF,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

pub type WSParseResult<T> = Result<T, WSParseError>;

pub trait WSParser {
    fn next_statement(&mut self) -> WSParseResult<WSStatement>;
}
