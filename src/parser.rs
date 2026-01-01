use std::io;

use crate::statements::{
    Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
    StatementStackManip,
};

///
/// GLOBAL PARSE ERROR
///
pub enum ParseError {
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
}
pub enum ParseErrorIO {
    UnexpectedEOF,
    ForbiddenLF,
}
pub enum ParseErrorStackManip {
    UnexpectedEOF,
    ForbiddenTab,
}
pub enum ParseErrorArithmetic {
    UnexpectedEOF,
    ForbiddenLF
}
pub enum ParseErrorFlowCtrl {
    UnexpectedEOF,
    WrongProgramEnd,
    ForbiddenLF,
}
pub enum ParseErrorHeapAccess {
    UnexpectedEOF,
}

///
/// DISPLAYING PARSE ERRORS
///
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parsing > ")?;

        match self {
            ParseError::IMP(err) => {
                write!(f, "imp > ")?;
                match err {
                    ParseErrorIMP::UnexpectedEOF => write!(f, "unexpected EOF"),
                }
            }
            ParseError::IO(err) => {
                write!(f, "io > ")?;
                match err {
                    ParseErrorIO::UnexpectedEOF => write!(f, "unexpected EOF"),
                    ParseErrorIO::ForbiddenLF => write!(f, "[LF] is not a valid command here"),
                }
            }
            ParseError::StackManip(err) => {
                write!(f, "stack manipulation > ")?;
                match err {
                    ParseErrorStackManip::UnexpectedEOF => write!(f, "unexpected EOF"),
                    ParseErrorStackManip::ForbiddenTab => {
                        write!(f, "[Tab] is not a valid command here")
                    }
                }
            }
            ParseError::Arithmetic(err) => {
                write!(f, "arithmetic > ")?;
                match err {
                    ParseErrorArithmetic::UnexpectedEOF => write!(f, "unexpected EOF"),
                    ParseErrorArithmetic::ForbiddenLF => write!(f, "[LF] is not a valid command here")
                }
            }
            ParseError::FlowCtrl(err) => {
                write!(f, "flow control > ")?;
                match err {
                    ParseErrorFlowCtrl::UnexpectedEOF => write!(f, "unexpected EOF"),
                    ParseErrorFlowCtrl::WrongProgramEnd => write!(f, "wrong program end"),
                    ParseErrorFlowCtrl::ForbiddenLF => {
                        write!(f, "[LF] is not a valid command here")
                    }
                }
            }
            ParseError::HeapAccess(err) => {
                write!(f, "head access > ")?;
                match err {
                    ParseErrorHeapAccess::UnexpectedEOF => write!(f, "unexpected EOF"),
                }
            }
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
                match value.kind() {
                    io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
                    _ => panic!("{}", value),
                }
            }
        }
    };
}
impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::IMP(ParseErrorIMP::UnexpectedEOF),
            _ => panic!("{}", value),
        }
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

///
/// TYPE ALIASES
///
pub type ParseResult = Result<Statement, ParseError>;
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
