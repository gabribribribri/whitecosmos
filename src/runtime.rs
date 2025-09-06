use crate::handler::Statement;

#[derive(Debug)]
pub enum RuntimeError {
    ReadEmptyStack,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime > {:?}", self)
    }
}

impl std::error::Error for RuntimeError {}

pub enum RuntimeAction {
    Next,
    EndProgram
}

pub type RuntimeResult = Result<RuntimeAction, RuntimeError>;

pub trait Runtime {
    fn run_statement(&mut self, statement: Statement) -> RuntimeResult;
}
