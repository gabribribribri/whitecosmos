mod handler;

mod statements;

mod parser;
mod ws_parser;

mod direct_runtime;
mod runtime;

use std::fs::File;
use std::io;


fn main() {
    let path: String = std::env::args().nth(1).unwrap();

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let parser = ws_parser::WSParser::new(reader);

    let runtime = direct_runtime::DirectRuntime::new();

    let handler = handler::Handler::new(Box::new(parser), Box::new(runtime));
    handler.run();
}
