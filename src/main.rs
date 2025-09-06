mod ws_handler;

mod ws_normal_parser;
mod ws_parser;

mod ws_direct_runtime;
mod ws_runtime;

use std::fs::File;
use std::io;


fn main() {
    let path: String = std::env::args().nth(1).unwrap();

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let parser = ws_normal_parser::WSNormalParser::new(reader);

    let runtime = ws_direct_runtime::WSDirectRuntime::new();

    let handler = ws_handler::WSHandler::new(Box::new(parser), Box::new(runtime));
    handler.run();
}
