use std::cell::RefCell;
use std::{fs::File, rc::Rc};

use std::io::{self, Write};

use whitecosmos::handler_errors::EngineError;
use whitecosmos::{
    classic_parser::{ClassicParser, TokenValues},
    direct_runtime::DirectRuntime,
    handler::Handler,
};

struct SharedBuffer(Rc<RefCell<Vec<u8>>>);

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.borrow_mut().flush()
    }
}

fn classic_direct_output_as_string(path: &'static str, tokens: TokenValues) -> Result<String, EngineError> {
    let file = File::open(path).unwrap();
    let reader = Box::new(io::BufReader::new(file));
    let parser = Box::new(ClassicParser::new(reader, tokens));
    let storage = Rc::new(RefCell::new(Vec::new()));
    let writer = SharedBuffer(storage.clone());
    let runtime = Box::new(DirectRuntime::new(Box::new(writer)));
    let mut handler = Handler::new(parser, runtime);
    handler.run()?;
    Ok(String::from_utf8(storage.borrow().to_vec()).unwrap())
}

const FAKE_WS_TOKENS: TokenValues = TokenValues {
    lf: b'l',
    tab: b't',
    space: b's',
};

mod classic_parser_direct_runtime {

    use super::*;

    #[test]
    fn hello_world() -> Result<(), EngineError> {
        let output = classic_direct_output_as_string("programs/hello_world.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "Hello, world");
        Ok(())
    }

    #[test]
    fn test1()-> Result<(), EngineError>{
        let output = classic_direct_output_as_string("programs/test1.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "abc\n2048\n12\n1\n521\n587654321\n");
        Ok(())
    }

    #[test]
    fn arithmetic()-> Result<(), EngineError>{
        // I was too bored to test Modulo and Multiplication
        let output =
            classic_direct_output_as_string("programs/arithmetic_tests.fws", FAKE_WS_TOKENS)?;
        assert_eq!(output, "1174746");
        Ok(())
    }
}
