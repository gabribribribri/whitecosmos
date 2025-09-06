use crate::{ws_parser::WSParser, ws_runtime::WSRuntime};

pub struct WSHandler {
    parser: Box<dyn WSParser>,
    runtime: Box<dyn WSRuntime>,
}


impl WSHandler {
    pub fn new(parser: Box<dyn WSParser>, runtime: Box<dyn WSRuntime>) -> Self {
        Self { parser, runtime }
    }

    pub fn run(mut self) {
        loop {
            let statement = self.parser.next_statement().unwrap();
            self.runtime.run_statement(statement).unwrap()
            
        }
    }
}
