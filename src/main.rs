use std::fs::File;
use std::io;

use whitecosmos::{direct_runtime, handler, classic_parser};

fn main() {
    let path: String = std::env::args().nth(1).unwrap();

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let tokens = classic_parser::TokenValues { lf: b'l', tab: b't', space: b's'};
    let parser = classic_parser::WSParser::new(reader, tokens);

    let runtime = direct_runtime::DirectRuntime::new(Box::new(std::io::stdout()));

    let mut handler = handler::Handler::new(parser, runtime);
    handler.run();
}
