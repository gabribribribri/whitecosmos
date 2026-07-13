use std::io;

use crate::{
    backend::runtime::{RuntimeError, RuntimeErrorFlowCtrl},
    frontend::parser::ParseError,
};

///
/// USAGE ERROR
///
pub enum UsageError {
    UnspecifiedParserType,
    UnsupportedFileExtension,
    MissingFilename,
    OsIoError(io::Error),
}

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UsageError::*;
        match self {
            UnspecifiedParserType => write!(f, "unspecified parser type"),
            UnsupportedFileExtension => write!(f, "unsupported file extension"),
            MissingFilename => write!(f, "missing file to execute"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
        }
    }
}

// impl From<io::Error> for UsageError {
//     fn from(value: io::Error) -> Self {
//         Self::OsIoError(value)
//     }
// }
impl From<io::Error> for EngineError {
    fn from(value: io::Error) -> Self {
        Self::usage(UsageError::OsIoError(value))
    }
}

impl std::fmt::Debug for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl From<UsageError> for EngineErrorKind {
    fn from(value: UsageError) -> Self {
        Self::Usage(value)
    }
}

///
/// ENGINE ERROR
///

pub enum EngineErrorKind {
    Parse(ParseError),
    Runtime(RuntimeError),
    Usage(UsageError),
}

impl From<io::Error> for EngineErrorKind {
    fn from(value: io::Error) -> Self {
        Self::Usage(UsageError::OsIoError(value))
    }
}

impl From<RuntimeErrorFlowCtrl> for EngineErrorKind {
    fn from(value: RuntimeErrorFlowCtrl) -> Self {
        Self::Runtime(RuntimeError::FlowCtrl(value))
    }
}

pub struct EngineError {
    location: usize,
    kind: EngineErrorKind,
}

impl EngineError {
    pub fn new<K>(location: usize, kind: K) -> Self
    where
        K: Into<EngineErrorKind>,
    {
        EngineError {
            location,
            kind: kind.into(),
        }
    }

    pub fn err<T, K>(location: usize, kind: K) -> Result<T, Self>
    where
        K: Into<EngineErrorKind>,
    {
        Err(Self::new(location, kind))
    }

    pub fn usage<K>(kind: K) -> Self
    where
        K: Into<EngineErrorKind>,
    {
        Self::new(0, kind)
    }
}

impl From<UsageError> for EngineError {
    fn from(value: UsageError) -> Self {
        Self::new(0, value)
    }
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Usage(_) = self.kind {
            write!(f,"\x1b[1;31merror\x1b[0m while parsing command line arguments :\n")?
        } else {
            write!(
                f,
                "\x1b[1;31merror\x1b[0m at statement \x1b[1m{}\x1b[0m :\n",
                self.location
            )?;
        }

        use EngineErrorKind::*;
        match self.kind {
            Parse(ref err) => write!(f, "\x1b[1;36mparsing > \x1b[0m{err}"),
            Runtime(ref err) => write!(f, "\x1b[1;36mruntime > \x1b[0m{err}"),
            Usage(ref err) => write!(f, "\x1b[1;36musage > \x1b[0m{err}"),
        }
    }
}

impl std::fmt::Debug for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::error::Error for EngineError {}
