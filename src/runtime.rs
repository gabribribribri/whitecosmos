use crate::statements::Statement;

///
/// RUNTIME
///
#[derive(Copy, Clone)]
pub enum RuntimeReport {
    Next,
    EndProgram,
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
    ReadEmptyStack,
    InvalidUTF8Character
}
pub enum RuntimeErrorStackManip {
    EmptyStack,
    StackTooSmall,
    NotInStackRange
}
pub enum RuntimeErrorArithmetic {
    NoRhsOnStack,
    NoLhsOnStack,
    DivisionByZero,
    UnderflowOrOverflow
}
pub enum RuntimeErrorFlowCtrl {}
pub enum RuntimeErrorHeapAccess {}

///
/// DISPLAYING RUNTIME ERRORS
///
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "runtime > ")?;

        use RuntimeError::*;
        match self {
            IO(err) => {
                write!(f, "io > ")?;
                match err {
                    RuntimeErrorIO::ReadEmptyStack => write!(f, "read empty stack"),
                    RuntimeErrorIO::InvalidUTF8Character => write!(f, "invalid UTF-8 character")
                }
            },
            StackManip(err) => {
                write!(f, "stack manipulation > ")?;
                match err {
                    RuntimeErrorStackManip::EmptyStack => write!(f, "empty stack"),
                    RuntimeErrorStackManip::StackTooSmall => write!(f, "stack too small"),
                    RuntimeErrorStackManip::NotInStackRange => write!(f, "not in stack range"),
                }
            },
            Arithmetic(err) => {
                write!(f, "arithmetic > ")?;
                match err {
                    RuntimeErrorArithmetic::NoRhsOnStack => write!(f, "empty stack, no possible operation"),
                    RuntimeErrorArithmetic::NoLhsOnStack => write!(f, "stack contains only one element"),
                    RuntimeErrorArithmetic::DivisionByZero => write!(f, "division by zero occured"),
                    RuntimeErrorArithmetic::UnderflowOrOverflow => write!(f, "overflow or underflow occured")
                    
                }
            },
            FlowCtrl(err) => {
                write!(f, "flow control > ")?;
                match err {
                    _ => todo!()
                }
            },
            HeapAccess(err) => {
                write!(f, "heap access > ")?;
                match err {
                    _ => todo!()
                }
            },
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
