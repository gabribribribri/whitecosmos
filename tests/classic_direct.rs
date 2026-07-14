use std::fs::File;

use whitecosmos::backend::runtime::SharedStorage;
use whitecosmos::core::handler_errors::EngineError;
use whitecosmos::frontend::classic_parser::FAKE_WS_TOKENS;
use whitecosmos::{
    backend::interpreter::Interpreter,
    core::handler::Handler,
    frontend::classic_parser::{ClassicParser, ParsedLanguage},
};

fn classic_direct_output_as_string(
    path: &'static str,
    tokens: ParsedLanguage,
) -> Result<String, EngineError> {
    let reader = Box::new(File::open(path)?);
    let parser = Box::new(ClassicParser::new(reader, tokens));
    let storage = SharedStorage::new();
    let runtime = Box::new(Interpreter::new(
        Box::new(std::io::stdin()),
        Box::new(storage.create_writer()),
    ));
    let mut handler = Handler::new(parser, runtime);
    handler.run()?;
    Ok(storage.data_as_string().unwrap())
}

mod classic_parser_direct_runtime {

    use super::*;

    #[test]
    fn hello_world() -> Result<(), EngineError> {
        let output = classic_direct_output_as_string("programs/hello_world.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "Hello, world");
        Ok(())
    }

    #[test]
    fn basic_features() -> Result<(), EngineError> {
        let output =
            classic_direct_output_as_string("programs/basic_features.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "abc\n2048\n12\n1\n521\n587654321\n");
        Ok(())
    }

    #[test]
    fn arithmetic() -> Result<(), EngineError> {
        // I was too bored to test Modulo and Multiplication
        let output =
            classic_direct_output_as_string("programs/arithmetic_tests.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "1174746");
        Ok(())
    }

    #[test]
    fn label_test() -> Result<(), EngineError> {
        let output = classic_direct_output_as_string("programs/label_test.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "16151413121110987654321");
        Ok(())
    }
}
