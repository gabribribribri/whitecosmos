use crate::{
    parser::{ParseError, Parser},
    runtime::{Runtime, RuntimeAction},
};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
    statements: Vec<Statement>,
    stat_index: usize,
}

#[derive(Clone, Copy)]
pub enum Statement {
    PopStackOutputNumber,
    EndProgram,
}

impl Handler {
    pub fn new(parser: Box<dyn Parser>, runtime: Box<dyn Runtime>) -> Self {
        Self {
            parser,
            runtime,
            statements: Vec::new(),
            stat_index: 0
        }
    }

    pub fn run(mut self) {
        loop {
            let statement = match self.read_statement() {
                Ok(st) => st,
                Err(e) => return self.print_error(Box::new(e)),
            };

            let action = match self.runtime.run_statement(statement) {
                Ok(act) => act,
                Err(e) => return self.print_error(Box::new(e)),
            };

            match action {
                RuntimeAction::Next => self.stat_index += 1,
                RuntimeAction::EndProgram => return,
            }
        }
    }

    fn read_statement(&mut self) -> Result<Statement, ParseError> {
        if self.stat_index < self.statements.len() {
            Ok(self.statements[self.stat_index])
        } else if self.stat_index == self.statements.len() {
            match self.parser.next_statement() {
                Ok(st) => {
                    self.statements.push(st);
                    Ok(st)
                }
                Err(e) => Err(e)
            }
        } else {
            panic!("This should not happen >:(")
        }
    }

    fn print_error(&self, err: Box<dyn std::error::Error>) {
        println!("\nEncountered error on statement {}", self.stat_index+1);
        println!("{}\n", err);
    }
}
