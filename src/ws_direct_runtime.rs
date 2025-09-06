use crate::{
    ws_parser::WSStatement,
    ws_runtime::{WSRuntime, WSRuntimeError, WSRuntimeResult},
};

pub struct WSDirectRuntime {
    stack: Vec<i32>,
}

impl WSDirectRuntime {
    pub fn new() -> Self {
        Self {
            stack: vec![1, 2, 3, 4242],
        }
    }
}

impl WSRuntime for WSDirectRuntime {
    fn run_statement(&mut self, statement: WSStatement) -> WSRuntimeResult<()> {
        use WSStatement::*;
        match statement {
            PopStackOutputNumber => {
                // Should we pop the last element ?
                match self.stack.pop() {
                    Some(i) => Ok(print!("{i}")),
                    None => Err(WSRuntimeError::ReadEmptyStack)
                }
            }
        }
    }
}
