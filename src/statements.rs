///
/// GLOBAL STATEMENTS
///
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum StatementStackManip {
    Push(i32),
    DuplicateTopItem,
    SwapTopTwoItems,
    DiscardTopItem,
    CopyNthOnTop(i32),
    SlideKeepTopItem(i32),
}
#[derive(Debug, Copy, Clone)]
pub enum StatementFlowCtrl {
    EndProgram,
}
#[derive(Debug, Copy, Clone)]
pub enum StatementHeapAccess {}
#[derive(Debug, Copy, Clone)]
pub enum StatementArithmetic {
    Addition,
    Substraction,
    Multiplication,
    IntegerDivision,
    Modulo,
}
#[derive(Debug, Copy, Clone)]
pub enum StatementIO {
    PopStackOutputNumber,
    PopStackOutputChar,
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
