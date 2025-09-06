use crate::{parser::Parser, runtime::Runtime};

pub struct Handler {
    parser: Box<dyn Parser>,
    runtime: Box<dyn Runtime>,
}


impl Handler {
    pub fn new(parser: Box<dyn Parser>, runtime: Box<dyn Runtime>) -> Self {
        Self { parser, runtime }
    }

    pub fn run(mut self) {
        loop {
            let statement = self.parser.next_statement().unwrap();
            self.runtime.run_statement(statement).unwrap()
            
        }
    }
}
