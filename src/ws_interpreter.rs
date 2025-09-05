use std::{
    error::Error,
    io::{self, Read},
    path::Display,
};

const LF: u8 = 0x6c;
const TAB: u8 = 0x74;
const SPACE: u8 = 0x73;
// const LF: u8 = 0x0A;
// const TAB: u8 = 0x9;
// const SPACE: u8 = 0x20;

#[derive(Debug)]
pub enum WSError {
    EOF,
    ParseIMP,
    ParseIO,
    ParseStackManip,
    ParseArithmetic,
    ParseFlowCtrl,
    ParseHeapAccess,
    Unknown(String),
}

impl std::fmt::Display for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for WSError {}

impl From<io::Error> for WSError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::UnexpectedEof => Self::EOF,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

type WSResult<T> = Result<T, WSError>;

pub struct WSInterpreter {
    code: Vec<u8>,
    reader: io::BufReader<std::fs::File>,
    stack: Vec<i32>,
}

impl WSInterpreter {
    pub fn new(reader: io::BufReader<std::fs::File>) -> Self {
        WSInterpreter {
            code: Vec::new(),
            reader,
            stack: vec![1, 2, 3, 4, 5],
        }
    }

    pub fn run(&mut self) -> WSResult<()> {
        let mut code_index = 0;

        loop {
            match self.eval_statement(code_index) {
                Ok(index) => code_index = index,
                Err(WSError::EOF) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    fn eval_statement(&mut self, code_index: usize) -> WSResult<usize> {
        // TODO make a `read_slice` function
        let first_two = (self.read_char(code_index)?, self.read_char(code_index + 1)?);
        println!("{:?}", first_two);
        match first_two {
            (TAB, LF) => self.eval_io(code_index + 2),
            (SPACE, _) => self.eval_stack_manipulation(code_index + 2),
            (TAB, SPACE) => self.eval_arithmetic(code_index + 2),
            (LF, _) => self.eval_flow_control(code_index + 2),
            (TAB, TAB) => self.eval_head_access(code_index + 2),
            _ => Err(WSError::ParseIMP),
        }
    }

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn read_char(&mut self, code_index: usize) -> WSResult<u8> {
        dbg!(&self.code);
        if self.code.len() <= code_index {
            
            dbg!(&self.code);
            let start = self.code.len();
            self.reader
                .read_exact(&mut self.code.as_mut_slice()[start..code_index + 1])?;
            dbg!(&self.code);
        }
        Ok(self.code[code_index])
    }

    fn eval_io(&mut self, mut code_index: usize) -> WSResult<usize> {
        let first_two = (self.read_char(code_index)?, self.read_char(code_index + 1)?);
        match first_two {
            (TAB, SPACE) => todo!(),
            (TAB, TAB) => todo!(),
            (SPACE, SPACE) => todo!(),
            (SPACE, TAB) => match self.stack.last() {
                Some(i) => {
                    print!("{}", i);
                    Ok(code_index + 2)
                }
                None => Err(WSError::Unknown("Empty Stack".to_owned())),
            },
            _ => Err(WSError::ParseIO),
        }
    }

    fn eval_stack_manipulation(&mut self, mut code_index: usize) -> WSResult<usize> {
        todo!()
    }

    fn eval_arithmetic(&mut self, mut code_index: usize) -> WSResult<usize> {
        todo!()
    }

    fn eval_flow_control(&mut self, mut code_index: usize) -> WSResult<usize> {
        todo!()
    }

    fn eval_head_access(&mut self, mut code_index: usize) -> WSResult<usize> {
        todo!()
    }
}
