use crate::ws_parser::WSStatement;

#[derive(Debug)]
pub enum WSRuntimeError {
    ReadEmptyStack,
}

pub type WSRuntimeResult<T> = Result<T, WSRuntimeError>;

pub trait WSRuntime {
    fn run_statement(&mut self, statement: WSStatement) -> WSRuntimeResult<()>;
}
