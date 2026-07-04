use std::fs::File;

use std::io;

use whitecosmos::{
    classic_parser::{self, TokenValues},
    direct_runtime, handler,
};

fn classic_direct_output_as_string(path: &'static str, tokens: TokenValues) -> String {
    let file = File::open(path).unwrap();
    let reader = Box::new(io::BufReader::new(file));
    let parser = classic_parser::WSParser::new(reader, tokens);
    let runtime = direct_runtime::DirectRuntime::new(Vec::new());
    let mut handler = handler::Handler::new(parser, runtime);
    handler.run();
    String::from_utf8(handler.into_parts().1.output).unwrap()
}

const FAKE_WS_TOKENS: TokenValues = TokenValues {
    lf: b'l',
    tab: b't',
    space: b's',
};

mod classic_parser_direct_runtime {

    use super::*;

    #[test]
    fn hello_world() {
        let output = classic_direct_output_as_string("programs/hello_world.fws", FAKE_WS_TOKENS);
        assert_eq!(output, "Hello, world");
    }

    #[test]
    fn test1() {
        let output = classic_direct_output_as_string("programs/test1.fws", FAKE_WS_TOKENS);
        assert_eq!(output, "abc\n2048\n12\n1\n521\n587654321\n");
    }

    #[test]
    fn arithmetic() {
        // I was too bored to test Modulo and Multiplication
        let output =
            classic_direct_output_as_string("programs/arithmetic_tests.fws", FAKE_WS_TOKENS);
        assert_eq!(output, "1174746");
    }
}
