use std::fs::File;


mod classic_parser_direct_runtime {
    use std::io;

    use whitecosmos::{direct_runtime, handler, classic_parser};

    use super::*;

    #[test]
    fn hello_world() {
        let file = File::open("programs/hello_world.fws").unwrap();
        let reader = io::BufReader::new(file);
        let parser = classic_parser::WSParser::new(reader);
        let runtime = direct_runtime::DirectRuntime::new(Vec::new());
        let mut handler = handler::Handler::new(parser, runtime);
        handler.run();
        let output = String::from_utf8(handler.into_parts().1.output).unwrap();
        assert_eq!(output, "Hello, world");
    }

    #[test]
    fn test1() {
        let file = File::open("programs/test1.fws").unwrap();
        let reader = io::BufReader::new(file);
        let parser = classic_parser::WSParser::new(reader);
        let runtime = direct_runtime::DirectRuntime::new(Vec::new());
        let mut handler = handler::Handler::new(parser, runtime);
        handler.run();
        let output = String::from_utf8(handler.into_parts().1.output).unwrap();
        assert_eq!(output, "abc\n2048\n12\n1\n521\n587654321\n");
    }

    #[test]
    fn arithmetic() {
        // I was too bored to test Modulo and Multiplication
        let file = File::open("programs/arithmetic_test.fws").unwrap();
        let reader = io::BufReader::new(file);
        let parser = classic_parser::WSParser::new(reader);
        let runtime = direct_runtime::DirectRuntime::new(Vec::new());
        let mut handler = handler::Handler::new(parser, runtime);
        handler.run();
        let output = String::from_utf8(handler.into_parts().1.output).unwrap();
        assert_eq!(output, "1174746");
    }
}
