use crate::{
    handler::Statement,
    runtime::{Runtime, RuntimeError, RuntimeResult, RuntimeAction},
};

pub struct DirectRuntime {
    stack: Vec<i32>,
}

impl DirectRuntime {
    pub fn new() -> Self {
        Self {
            stack: vec![1, 2, 3, 4242],
            // stack: Vec::new(),
        }
    }
}

impl Runtime for DirectRuntime {
    fn run_statement(&mut self, statement: Statement) -> RuntimeResult {
        use Statement::*;
        match statement {
            PopStackOutputNumber => self.pop_stack_output_number(),
            EndProgram => Ok(RuntimeAction::EndProgram)
        }
    }
}

impl DirectRuntime {
    fn pop_stack_output_number(&mut self) -> RuntimeResult {
        // Should we pop the last element ?
        match self.stack.pop() {
            Some(i) => {print!("{i}"); Ok(RuntimeAction::Next)},
            None => Err(RuntimeError::ReadEmptyStack),
        }
    }
}
