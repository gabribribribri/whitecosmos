use crate::{core::handler_errors::EngineError, core::statements::Statement};

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
    HeapAccess(RuntimeErrorHeapAccess),
}

///
/// LOCAL RUNTIME ERRORS
///
pub enum RuntimeErrorIO {
    EmptyStack,
    InvalidUTF8Character,
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

pub enum RuntimeErrorHeapAccess {}

///
/// DISPLAYING RUNTIME ERRORS
///
impl std::fmt::Display for RuntimeErrorIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RuntimeErrorIO::*;
        match self {
            EmptyStack => write!(f, "read empty stack"),
            InvalidUTF8Character => write!(f, "invalid UTF-8 character"),
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

impl std::fmt::Display for RuntimeErrorHeapAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
            HeapAccess(err) => write!(f, "heap access > {}", err),
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
impl_from_for_runtime_error!(RuntimeErrorHeapAccess, HeapAccess);
impl_from_for_runtime_error!(RuntimeErrorFlowCtrl, FlowCtrl);
impl_from_for_runtime_error!(RuntimeErrorArithmetic, Arithmetic);
impl_from_for_runtime_error!(RuntimeErrorStackManip, StackManip);

impl From<RuntimeError> for EngineError {
    fn from(value: RuntimeError) -> Self {
        Self::Runtime(value)
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
pub type RuntimeResultHeapAccess = Result<RuntimeReport, RuntimeErrorHeapAccess>;

///
/// Actual trait
///
pub trait Runtime {
    fn run_statement(&mut self, statement: Statement) -> Result<RuntimeReport, RuntimeError>;
}
