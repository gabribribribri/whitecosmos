use std::fs::File;
use std::io;

use whitecosmos::{direct_runtime, handler, ws_parser};

fn main() {
    let path: String = std::env::args().nth(1).unwrap();

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let parser = ws_parser::WSParser::new(reader);

    let runtime = direct_runtime::DirectRuntime::new(Box::new(std::io::stdout()));

    let mut handler = handler::Handler::new(parser, runtime);
    handler.run();
}
