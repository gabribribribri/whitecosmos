use crate::{
    parser::Statement,
    runtime::{Runtime, RuntimeError, RuntimeResult},
};

pub struct DirectRuntime {
    stack: Vec<i32>,
}

impl DirectRuntime {
    pub fn new() -> Self {
        Self {
            stack: vec![1, 2, 3, 4242],
        }
    }
}

impl Runtime for DirectRuntime {
    fn run_statement(&mut self, statement: Statement) -> RuntimeResult<()> {
        use Statement::*;
        match statement {
            PopStackOutputNumber => {
                // Should we pop the last element ?
                match self.stack.pop() {
                    Some(i) => Ok(print!("{i}")),
                    None => Err(RuntimeError::ReadEmptyStack)
                }
            }
        }
    }
}
