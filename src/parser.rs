use crate::handler::Statement;
use std::io;

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
    NotTabNorSpace,
}
pub enum ParseErrorStackManip {
    UnexpectedEOF,
}
pub enum ParseErrorArithmetic {
    UnexpectedEOF,
}
pub enum ParseErrorFlowCtrl {
    UnexpectedEOF,
    WrongProgramEnd,
    DisallowedCharAfterTab,
}
pub enum ParseErrorHeapAccess {
    UnexpectedEOF,
}

///
/// DISPLAYING PARSE ERRORS
///
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing > ");

        match self {
            ParseError::IMP(err) => {
                write!(f, "IMP > ");
                match err {
                    ParseErrorIMP::UnexpectedEOF => write!(f, "Unexpected EOF"),
                }
            }
            ParseError::IO(err) => {
                write!(f, "IO > ");
                match err {
                    ParseErrorIO::UnexpectedEOF => write!(f, "Unexpected EOF"),
                    ParseErrorIO::NotTabNorSpace => write!(f, "Not a [Tab] nor a [Space]"),
                }
            }
            ParseError::StackManip(err) => {
                write!(f, "Stack Manipulation > ");
                match err {
                    ParseErrorStackManip::UnexpectedEOF => write!(f, "Unexpected EOF"),
                }
            }
            ParseError::Arithmetic(err) => {
                write!(f, "Arithmetic > ");
                match err {
                    ParseErrorArithmetic::UnexpectedEOF => write!(f, "Unexpected EOF"),
                }
            }
            ParseError::FlowCtrl(err) => {
                write!(f, "Flow Control > ");
                match err {
                    ParseErrorFlowCtrl::UnexpectedEOF => write!(f, "Unexpected EOF"),
                    ParseErrorFlowCtrl::WrongProgramEnd => write!(f, "Wrong Program End"),
                    ParseErrorFlowCtrl::DisallowedCharAfterTab => {
                        write!(f, "Character disallowed after a [Tab] here")
                    }
                }
            }
            ParseError::HeapAccess(err) => {
                write!(f, "Head Access > ");
                match err {
                    ParseErrorHeapAccess::UnexpectedEOF => write!(f, "Unexpected EOF"),
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
impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::IMP(ParseErrorIMP::UnexpectedEOF),
            _ => panic!("{}", value),
        }
    }
}
impl From<io::Error> for ParseErrorIO {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => panic!("{}", value),
        }
    }
}
impl From<io::Error> for ParseErrorStackManip {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => panic!("{}", value),
        }
    }
}
impl From<io::Error> for ParseErrorHeapAccess {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => panic!("{}", value),
        }
    }
}
impl From<io::Error> for ParseErrorFlowCtrl {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => panic!("{}", value),
        }
    }
}
impl From<io::Error> for ParseErrorArithmetic {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEOF,
            _ => panic!("{}", value),
        }
    }
}

///
/// CONVERTING LOCAL PARSING ERRORS
///  LocalParseError -> GlobalParseError
///
impl From<ParseErrorIO> for ParseError {
    fn from(value: ParseErrorIO) -> Self {
        Self::IO(value)
    }
}
impl From<ParseErrorHeapAccess> for ParseError {
    fn from(value: ParseErrorHeapAccess) -> Self {
        Self::HeapAccess(value)
    }
}
impl From<ParseErrorFlowCtrl> for ParseError {
    fn from(value: ParseErrorFlowCtrl) -> Self {
        Self::FlowCtrl(value)
    }
}
impl From<ParseErrorArithmetic> for ParseError {
    fn from(value: ParseErrorArithmetic) -> Self {
        Self::Arithmetic(value)
    }
}
impl From<ParseErrorStackManip> for ParseError {
    fn from(value: ParseErrorStackManip) -> Self {
        Self::StackManip(value)
    }
}

///
/// Actual trait...
///
pub trait Parser {
    fn next_statement(&mut self) -> Result<Statement, ParseError>;
}
