use std::io;

use crate::{parser::ParseError, runtime::RuntimeError};


///
/// USAGE ERROR
/// 
pub enum UsageError {
    UnspecifiedParserType,
    UnsupportedFileExtension,
    IoError(String),
}

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UsageError::*;
        match self {
            UnspecifiedParserType => write!(f, "unspecified parser type"),
            UnsupportedFileExtension => write!(f, "unsupported file extension"),
            IoError(s) => write!(f, "IO Error > {}", s)
        }
    }
}

impl std::fmt::Debug for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<UsageError> for EngineError {
    fn from(value: UsageError) -> Self {
        Self::Usage(value)
    }
}


///
/// ENGINE ERROR
/// 
pub enum EngineError {
    Parse(ParseError),
    Runtime(RuntimeError),
    Usage(UsageError),
}


impl From<io::Error> for EngineError {
    fn from(value: io::Error) -> Self {
        Self::Usage(UsageError::IoError(value.to_string()))
    }
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n-!!!- ENCOUNTERED UNRECOVERABLE ERROR -!!!-\n\n")?;

        match self {
            EngineError::Parse(err) => write!(f, "parsing > {err}"),
            EngineError::Runtime(err) => write!(f, "runtime > {err}"),
            EngineError::Usage(err) => write!(f, "usage > {err}"),
        }
    }
}


impl std::fmt::Debug for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for EngineError {}
