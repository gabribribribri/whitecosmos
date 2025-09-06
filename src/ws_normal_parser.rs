use std::io::{self, Read};

use crate::ws_parser::{WSParseError, WSParseResult, WSParser, WSStatement};

const LF: u8 = 0x6c;
const TAB: u8 = 0x74;
const SPACE: u8 = 0x73;
// const LF: u8 = 0x0A;
// const TAB: u8 = 0x9;
// const SPACE: u8 = 0x20;

pub struct WSNormalParser {
    code: Vec<u8>,
    // code_index is the LAST READ character. NOT the next one to read
    code_index: usize,
    reader: io::BufReader<std::fs::File>,
}

impl WSParser for WSNormalParser {
    fn next_statement(&mut self) -> WSParseResult<WSStatement> {
        loop {
            self.code_index += 1;
            match self.index_char()? {
                TAB => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        TAB => return self.parse_head_access(),
                        SPACE => return self.parse_arithmetic(),
                        LF => return self.parse_io(),
                        _ => (),
                    }
                },
                SPACE => return self.parse_stack_manipulation(),
                LF => return self.parse_flow_control(),
                _ => (),
            }
        }
    }
}

impl WSNormalParser {
    pub fn new(reader: io::BufReader<std::fs::File>) -> Self {
        WSNormalParser {
            code: vec![69],
            code_index: 0,
            reader,
        }
    }

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn index_char(&mut self) -> WSParseResult<u8> {
        while self.code.len() <= self.code_index {
            let mut trans_buf = [0u8; 512];
            let nb_read = self.reader.read(&mut trans_buf)?;
            if nb_read == 0 {
                return Err(WSParseError::EOF);
            }
            self.code.extend_from_slice(&trans_buf[..nb_read]);
        }
        Ok(self.code[self.code_index])
    }

    fn parse_io(&mut self) -> WSParseResult<WSStatement> {
        loop {
            self.code_index += 1;
            match self.index_char()? {
                TAB => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        TAB => todo!(),
                        SPACE => todo!(),
                        _ => (),
                    }
                },
                SPACE => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        TAB => return Ok(WSStatement::PopStackOutputNumber),
                        SPACE => todo!(),
                        _ => (),
                    }
                },
                LF => return Err(WSParseError::IO),
                _ => (),
            }
        }
    }

    fn parse_stack_manipulation(&mut self) -> WSParseResult<WSStatement> {
        todo!()
    }

    fn parse_arithmetic(&mut self) -> WSParseResult<WSStatement> {
        todo!()
    }

    fn parse_flow_control(&mut self) -> WSParseResult<WSStatement> {
        todo!()
    }

    fn parse_head_access(&mut self) -> WSParseResult<WSStatement> {
        todo!()
    }
}
