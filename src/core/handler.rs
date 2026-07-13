use std::collections::HashMap;

use crate::{
    backend::runtime::{Runtime, RuntimeErrorFlowCtrl, RuntimeReport},
    core::{
        handler_errors::{EngineError, EngineErrorKind},
        statements::{Statement, StatementFlowCtrl},
    },
    frontend::parser::{ParseError, Parser},
};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
    statements: Vec<Statement>,
    labels: HashMap<i32, usize>,
    callstack: Vec<usize>,
}

type ProgramContinue = bool;

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
            let statement = match self.read_statement(stat_index) {
                Ok(stat) => stat,
                Err(e) => return EngineError::err(stat_index, e),
            };

            let action = match self.runtime.run_statement(statement) {
                Ok(act) => act,
                Err(e) => return EngineError::err(stat_index, e),
            };

            match self.handle_runtime_report(action, &mut stat_index) {
                Ok(false) => return Ok(()),
                Ok(true) => (),
                Err(e) => return EngineError::err(stat_index, e),
            }

            stat_index += 1;
        }
    }

    ///
    /// Returns a Result of if the program will continue and the appropriate error
    ///
    fn handle_runtime_report(
        &mut self,
        action: RuntimeReport,
        stat_index: &mut usize,
    ) -> Result<ProgramContinue, EngineErrorKind> {
        use RuntimeReport::*;
        match action {
            Next => Ok(true),
            EndProgram => return Ok(false),
            MarkLabel(label) => {
                _ = self.labels.insert(label, *stat_index);
                Ok(true)
            }
            JumpTo(label) => match self.labels.get(&label) {
                Some(location) => {
                    *stat_index = *location;
                    Ok(true)
                }
                None => match self.hunt_label(label)? {
                    Some(location) => {
                        *stat_index = location;
                        Ok(true)
                    }
                    None => return Err(RuntimeErrorFlowCtrl::LabelNotFound.into()),
                },
            },
            CallSubroutine(label) => match self.labels.get(&label) {
                Some(location) => {
                    self.callstack.push(*stat_index);
                    *stat_index = *location;
                    Ok(true)
                }
                None => return Err(RuntimeErrorFlowCtrl::LabelNotFound.into()),
            },
            ReturnFromSubroutine => match self.callstack.last() {
                Some(location) => {
                    *stat_index = *location;
                    Ok(true)
                }
                None => return Err(RuntimeErrorFlowCtrl::EmptyCallStack.into()),
            },
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

    fn read_statement(&mut self, stat_index: usize) -> Result<Statement, ParseError> {
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
