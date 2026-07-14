use std::{
    cell::RefCell,
    io::{self, Write},
    num::ParseIntError,
    rc::Rc,
    str::Utf8Error,
    string::FromUtf8Error,
};

use crate::core::{handler_errors::EngineErrorKind, statements::Statement};

///
/// RUNTIME
///
#[derive(Copy, Clone)]
pub enum RuntimeReport {
    Next,
    EndProgram,
    MarkLabel(i32),
    JumpTo(i32),
    CallSubroutine(i32),
    ReturnFromSubroutine,
}

///
/// GLOBAL RUNTIME ERROR
///
pub enum RuntimeError {
    IO(RuntimeErrorIO),
    StackManip(RuntimeErrorStackManip),
    Arithmetic(RuntimeErrorArithmetic),
    FlowCtrl(RuntimeErrorFlowCtrl),
    Heap(RuntimeErrorHeap),
}

///
/// LOCAL RUNTIME ERRORS
///
pub enum RuntimeErrorIO {
    EmptyStack,
    InvalidUtf8StartByte,
    InvalidStoredUtf8Character,
    ParseUtf8(Utf8Error),
    EmptyInput,
    OsIoError(io::Error),
    ParseIntError(ParseIntError),
}

pub enum RuntimeErrorStackManip {
    EmptyStack,
    StackTooSmall,
    NotInStackRange,
}

pub enum RuntimeErrorArithmetic {
    NoRhsOnStack,
    NoLhsOnStack,
    DivisionByZero,
    UnderflowOrOverflow,
}
pub enum RuntimeErrorFlowCtrl {
    EmptyStack,
    LabelNotFound,
    EmptyCallStack,
}

pub enum RuntimeErrorHeap {
    EmptyStack,
    StackTooSmall,
    NothingAtAddress,
}

///
/// DISPLAYING RUNTIME ERRORS
///
impl std::fmt::Display for RuntimeErrorIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorIO::*;
        match self {
            EmptyStack => write!(f, "read empty stack"),
            ParseUtf8(err) => write!(f, "parsing utf-8 text > {}", err),
            EmptyInput => write!(f, "empty input"),
            OsIoError(err) => write!(f, "OS level IO error > {}", err),
            Self::ParseIntError(err) => write!(f, "error while parsing int > {}", err),
            InvalidStoredUtf8Character => write!(f, "wrong utf-8 character from the stack"),
            InvalidUtf8StartByte => write!(f, "read invalid utf-8 start byte"),
        }
    }
}

impl std::fmt::Display for RuntimeErrorStackManip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorStackManip::*;
        match self {
            EmptyStack => write!(f, "empty stack"),
            StackTooSmall => write!(f, "stack too small"),
            NotInStackRange => write!(f, "not in stack range"),
        }
    }
}

impl std::fmt::Display for RuntimeErrorArithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorArithmetic::*;
        match self {
            NoRhsOnStack => write!(f, "empty stack, no operation possible"),
            NoLhsOnStack => write!(f, "stack contains only one element"),
            DivisionByZero => write!(f, "division by zero occured"),
            UnderflowOrOverflow => write!(f, "overflow or underflow occured"),
        }
    }
}

impl std::fmt::Display for RuntimeErrorFlowCtrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorFlowCtrl::*;
        match self {
            EmptyStack => write!(f, "empty stack, no call possible"),
            EmptyCallStack => write!(f, "empty call stack, cannot return from subroutine"),
            LabelNotFound => write!(f, "label not found"),
        }
    }
}

impl std::fmt::Display for RuntimeErrorHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorHeap::*;
        match self {
            EmptyStack => write!(f, "empty stackk, unable to store"),
            StackTooSmall => write!(f, "stack too small, unable to find an address to store to"),
            NothingAtAddress => write!(f, "nothing at address"),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeError::*;
        match self {
            IO(err) => write!(f, "io > {}", err),
            StackManip(err) => write!(f, "stack manipulation > {err}"),
            Arithmetic(err) => write!(f, "arithmetic > {err}"),
            FlowCtrl(err) => write!(f, "flow control > {}", err),
            Heap(err) => write!(f, "heap > {}", err),
        }
    }
}

impl std::fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for RuntimeError {}

///
/// CONVERTING LOCAL RUNTIME ERRORS
///  LocalRuntimeError -> GlobalRuntimeError
///
macro_rules! impl_from_for_runtime_error {
    ($runtime_error_thing:ident, $thing:ident) => {
        impl From<$runtime_error_thing> for RuntimeError {
            fn from(value: $runtime_error_thing) -> Self {
                Self::$thing(value)
            }
        }
    };
}
impl_from_for_runtime_error!(RuntimeErrorIO, IO);
impl_from_for_runtime_error!(RuntimeErrorHeap, Heap);
impl_from_for_runtime_error!(RuntimeErrorFlowCtrl, FlowCtrl);
impl_from_for_runtime_error!(RuntimeErrorArithmetic, Arithmetic);
impl_from_for_runtime_error!(RuntimeErrorStackManip, StackManip);

impl From<RuntimeError> for EngineErrorKind {
    fn from(value: RuntimeError) -> Self {
        Self::Runtime(value)
    }
}

impl From<io::Error> for RuntimeErrorIO {
    fn from(value: io::Error) -> Self {
        RuntimeErrorIO::OsIoError(value)
    }
}

impl From<ParseIntError> for RuntimeErrorIO {
    fn from(value: ParseIntError) -> Self {
        RuntimeErrorIO::ParseIntError(value)
    }
}

///
/// TYPE ALIASES
///
pub type RuntimeResult = Result<RuntimeReport, RuntimeError>;
pub type RuntimeResultIO = Result<RuntimeReport, RuntimeErrorIO>;
pub type RuntimeResultFlowCtrl = Result<RuntimeReport, RuntimeErrorFlowCtrl>;
pub type RuntimeResultArithmetic = Result<RuntimeReport, RuntimeErrorArithmetic>;
pub type RuntimeResultStackManip = Result<RuntimeReport, RuntimeErrorStackManip>;
pub type RuntimeResultHeapAccess = Result<RuntimeReport, RuntimeErrorHeap>;

///
/// Actual trait
///
pub trait Runtime {
    fn run_statement(&mut self, statement: Statement) -> Result<RuntimeReport, RuntimeError>;
}

///
/// Proxy struct to retrieve data written to by the runtime
///
#[derive(Clone)]
pub struct SharedStorage {
    val: Rc<RefCell<Vec<u8>>>,
}

impl SharedStorage {
    pub fn new() -> Self {
        SharedStorage {
            val: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn data(&self) -> Vec<u8> {
        self.val.borrow().clone()
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data())
    }

    pub fn create_writer(&self) -> Box<Self> {
        Box::new(self.clone())
    }
}

impl Write for SharedStorage {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.val.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.val.borrow_mut().flush()
    }
}
