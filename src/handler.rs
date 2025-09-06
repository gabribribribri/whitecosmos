use crate::{
    parser::Parser,
    runtime::{Runtime, RuntimeAction},
};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
}

pub enum Statement {
    PopStackOutputNumber,
    EndProgram,
}

impl Handler {
    pub fn new(parser: Box<dyn Parser>, runtime: Box<dyn Runtime>) -> Self {
        Self { parser, runtime }
    }

    pub fn run(mut self) {
        loop {
            let statement = match self.parser.next_statement() {
                Ok(st) => st,
                Err(e) => return self.print_error(Box::new(e)),
            };

            let action = match self.runtime.run_statement(statement) {
                Ok(act) => act,
                Err(e) => return self.print_error(Box::new(e)),
            };

            match action {
                RuntimeAction::Next => (),
                RuntimeAction::EndProgram => return
            }
        }
    }

    fn print_error(&self, err: Box<dyn std::error::Error>) {
        println!("\nEncountered error :");
        println!("{}\n", err);
    }

}
