use crate::parser::Statement;

#[derive(Debug)]
pub enum RuntimeError {
    ReadEmptyStack,
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub trait Runtime {
    fn run_statement(&mut self, statement: Statement) -> RuntimeResult<()>;
}
