use std::collections::HashMap;

use crate::{
    backend::runtime::{Runtime, RuntimeError, RuntimeErrorFlowCtrl, RuntimeReport},
    core::{
        handler_errors::EngineError,
        statements::{Statement, StatementFlowCtrl},
    },
    frontend::parser::{ParseError, ParseResult, Parser},
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
                Next => (),
                EndProgram => return Ok(()),
                MarkLabel(label) => _ = self.labels.insert(label, stat_index),
                JumpTo(label) => match self.labels.get(&label) {
                    Some(location) => stat_index = *location,
                    None => match self.hunt_label(label)? {
                        Some(location) => stat_index = location,
                        None => {
                            return Err(EngineError::Runtime(RuntimeError::FlowCtrl(
                                RuntimeErrorFlowCtrl::LabelNotFound,
                            )));
                        }
                    },
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
            stat_index += 1;
        }
    }

    fn hunt_label(&mut self, label_to_find: i32) -> Result<Option<usize>, ParseError> {
        for running_index in self.statements.len().. {
            use {Statement::*, StatementFlowCtrl::*};
            match self.read_statement(running_index)? {
                FlowCtrl(MarkLabel(label)) => {
                    self.labels.insert(label, running_index);
                    if label == label_to_find {
                        return Ok(Some(running_index));
                    }
                }
                FlowCtrl(EndProgram) => return Ok(None),
                _ => (),
            }
        }
        todo!() // TODO find a way to exit properly
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
