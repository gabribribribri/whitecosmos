use crate::{
    handler_errors::EngineError,
    parser::{ParseResult, Parser},
    runtime::{Runtime, RuntimeReport},
    statements::Statement,
};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
    statements: Vec<Statement>,
    // maybe we will need to take back stat_index
    // stat_index: usize
}

impl Handler {
    pub fn new(parser: Box<dyn Parser>, runtime: Box<dyn Runtime>) -> Self {
        Self {
            parser,
            runtime,
            statements: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), EngineError> {
        let mut stat_index = 0;
        loop {
            let statement = self.read_statement(stat_index)?;

            let action = self.runtime.run_statement(statement)?;

            match action {
                RuntimeReport::Next => stat_index += 1,
                RuntimeReport::EndProgram => return Ok(()),
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
