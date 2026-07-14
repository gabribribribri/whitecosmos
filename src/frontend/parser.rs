use std::io;

use crate::core::{
    handler_errors::EngineErrorKind,
    statements::{
        Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
        StatementStackManip,
    },
};

///
/// GLOBAL PARSE ERROR
///
pub enum ParseError {
    UnexpectedEof,
    OsIoError(io::Error),
    IMP(ParseErrorIMP),
    IO(ParseErrorIO),
    StackManip(ParseErrorStackManip),
    Arithmetic(ParseErrorArithmetic),
    FlowCtrl(ParseErrorFlowCtrl),
    HeapAccess(ParseErrorHeapAccess),
}

///
/// LOCAL PARSE ERRORS
///
pub enum ParseErrorIMP {
    UnexpectedEOF,
    OsIoError(io::Error),
}
pub enum ParseErrorIO {
    ForbiddenLF,
    OsIoError(io::Error),
}
pub enum ParseErrorStackManip {
    ForbiddenTab,
    OsIoError(io::Error),
}
pub enum ParseErrorArithmetic {
    ForbiddenLF,
    OsIoError(io::Error),
}
pub enum ParseErrorFlowCtrl {
    WrongProgramEnd,
    ForbiddenLF,
    OsIoError(io::Error),
}
pub enum ParseErrorHeapAccess {
    ForbiddenLF,
    OsIoError(io::Error),
}

///
/// DISPLAYING PARSE ERRORS
///

impl std::fmt::Display for ParseErrorIMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorIMP::*;
        match self {
            UnexpectedEOF => write!(f, "unexpected EOF"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseErrorIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorIO::*;
        match self {
            ForbiddenLF => write!(f, "[LF] is not a valid command here"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseErrorStackManip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorStackManip::*;
        match self {
            ForbiddenTab => write!(f, "[Tab] is not a valid command here"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseErrorArithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorArithmetic::*;
        match self {
            ForbiddenLF => write!(f, "[LF] is not a valid command here"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseErrorFlowCtrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorFlowCtrl::*;
        match self {
            WrongProgramEnd => write!(f, "wrong program end"),
            ForbiddenLF => write!(f, "[LF] is not a valid command here"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseErrorHeapAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseErrorHeapAccess::*;
        match self {
            ForbiddenLF => write!(f, "[LF] is not a valid command here"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            IMP(err) => write!(f, "imp > {err}"),
            IO(err) => write!(f, "io > {err}"),
            StackManip(err) => write!(f, "stack manipulation > {err}"),
            Arithmetic(err) => write!(f, "arithmetic > {err}"),
            FlowCtrl(err) => write!(f, "flow control > {err}"),
            HeapAccess(err) => write!(f, "head access > {err}"),
            UnexpectedEof => write!(f, "unexpected end of file"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ParseError {}

///
/// CONVERTING LOCAL PARSING ERRORS
///  io::Error -> LocalParseError
///
macro_rules! impl_ioerror_for {
    ($thing:ident) => {
        impl From<io::Error> for $thing {
            fn from(value: io::Error) -> Self {
                Self::OsIoError(value)
            }
        }
    };
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        Self::IMP(ParseErrorIMP::OsIoError(value))
    }
}

impl_ioerror_for!(ParseErrorIO);
impl_ioerror_for!(ParseErrorHeapAccess);
impl_ioerror_for!(ParseErrorStackManip);
impl_ioerror_for!(ParseErrorFlowCtrl);
impl_ioerror_for!(ParseErrorArithmetic);

///
/// CONVERTING LOCAL PARSING ERRORS
///  LocalParseError -> GlobalParseError
///
macro_rules! impl_from_for_parse_error {
    ($parse_error_thing:ident, $thing:ident) => {
        impl From<$parse_error_thing> for ParseError {
            fn from(value: $parse_error_thing) -> Self {
                Self::$thing(value)
            }
        }
    };
}
impl_from_for_parse_error!(ParseErrorIO, IO);
impl_from_for_parse_error!(ParseErrorHeapAccess, HeapAccess);
impl_from_for_parse_error!(ParseErrorFlowCtrl, FlowCtrl);
impl_from_for_parse_error!(ParseErrorArithmetic, Arithmetic);
impl_from_for_parse_error!(ParseErrorStackManip, StackManip);

impl From<ParseError> for EngineErrorKind {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

///
/// TYPE ALIASES
///
pub type ParseResultIO = Result<StatementIO, ParseErrorIO>;
pub type ParseResultFlowCtrl = Result<StatementFlowCtrl, ParseErrorFlowCtrl>;
pub type ParseResultArithmetic = Result<StatementArithmetic, ParseErrorArithmetic>;
pub type ParseResultStackManip = Result<StatementStackManip, ParseErrorStackManip>;
pub type ParseResultHeapAccess = Result<StatementHeapAccess, ParseErrorHeapAccess>;

///
/// Actual trait...
///
pub trait Parser {
    fn next_statement(&mut self) -> Result<Statement, ParseError>;
}

///
/// Different Tokens
///
pub enum TokenKind {
    Lf,
    Tab,
    Space,
}
