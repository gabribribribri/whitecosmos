use serde::{Deserialize, Serialize};
use wincode::{SchemaRead, SchemaWrite};

///
/// GLOBAL STATEMENTS
///
#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum Statement {
    IO(StatementIO),
    FlowCtrl(StatementFlowCtrl),
    HeapAccess(StatementHeapAccess),
    Arithmetic(StatementArithmetic),
    StackManip(StatementStackManip),
}

///
/// LOCAL STATEMENTS
///
#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum StatementStackManip {
    Push(i32),
    DuplicateTopItem,
    SwapTopTwoItems,
    DiscardTopItem,
    CopyNthOnTop(i32),
    SlideKeepTopItem(i32),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum StatementFlowCtrl {
    EndProgram,
    MarkLabel(i32),
    JumpTo(i32),
    JumpToIfZero(i32),
    JumpToIfNegative(i32),
    CallSubroutine(i32),
    ReturnFromSubroutine,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum StatementHeapAccess {
    Store,
    Retrieve,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum StatementArithmetic {
    Addition,
    Substraction,
    Multiplication,
    IntegerDivision,
    Modulo,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, SchemaRead, SchemaWrite)]
pub enum StatementIO {
    PopStackOutputNumber,
    PopStackOutputChar,
    ReadNumberStoreOnHeap,
    ReadCharStoreOnHeap,
}

///
/// CONVERTING LOCAL RUNTIME ERRORS
///  LocalRuntimeError -> GlobalRuntimeError
///
macro_rules! impl_from_for_statements {
    ($runtime_error_thing:ident, $thing:ident) => {
        impl From<$runtime_error_thing> for Statement {
            fn from(value: $runtime_error_thing) -> Self {
                Self::$thing(value)
            }
        }
    };
}

impl_from_for_statements!(StatementIO, IO);
impl_from_for_statements!(StatementHeapAccess, HeapAccess);
impl_from_for_statements!(StatementFlowCtrl, FlowCtrl);
impl_from_for_statements!(StatementArithmetic, Arithmetic);
impl_from_for_statements!(StatementStackManip, StackManip);
