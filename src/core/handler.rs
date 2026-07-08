use std::collections::HashMap;

use crate::{
    core::handler_errors::EngineError,
    frontend::parser::{ParseResult, Parser},
    backend::runtime::{Runtime, RuntimeError, RuntimeErrorFlowCtrl, RuntimeReport},
    core::statements::Statement,
};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
    statements: Vec<Statement>,
    labels: HashMap<i32, usize>,
    callstack: Vec<usize>,
}

impl Handler {
    pub fn new(parser: Box<dyn Parser>, runtime: Box<dyn Runtime>) -> Self {
        Self {
            parser,
            runtime,
            statements: Vec::new(),
            labels: HashMap::new(),
            callstack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), EngineError> {
        let mut stat_index = 0;
        loop {
            let statement = self.read_statement(stat_index)?;

            let action = self.runtime.run_statement(statement)?;

            use RuntimeReport::*;
            match action {
                Next => stat_index += 1,
                EndProgram => return Ok(()),
                MarkLabel(label) => _ = self.labels.insert(label, stat_index),
                JumpTo(label) => match self.labels.get(&label) {
                    Some(location) => stat_index = *location,
                    None => {
                        return Err(EngineError::Runtime(RuntimeError::FlowCtrl(
                            RuntimeErrorFlowCtrl::LabelNotFound,
                        )));
                    }
                },
                CallSubroutine(label) => match self.labels.get(&label) {
                    Some(location) => {
                        self.callstack.push(stat_index);
                        stat_index = *location;
                    }
                    None => {
                        return Err(EngineError::Runtime(RuntimeError::FlowCtrl(
                            RuntimeErrorFlowCtrl::LabelNotFound,
                        )));
                    }
                },
                ReturnFromSubroutine => match self.callstack.last() {
                    Some(location) => stat_index = *location,
                    None => {
                        return Err(EngineError::Runtime(RuntimeError::FlowCtrl(
                            RuntimeErrorFlowCtrl::EmptyCallStack,
                        )));
                    }
                },
            }
        }
    }

    fn read_statement(&mut self, stat_index: usize) -> ParseResult {
        if stat_index < self.statements.len() {
            Ok(self.statements[stat_index])
        } else if stat_index == self.statements.len() {
            match self.parser.next_statement() {
                Ok(st) => {
                    self.statements.push(st);
                    Ok(st)
                }
                Err(e) => Err(e),
            }
        } else {
            panic!("This should not happen >:(")
        }
    }
}
