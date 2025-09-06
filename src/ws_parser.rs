use std::io::{self, Read};

use crate::parser::{ParseError, ParseResult, Parser, Statement};

const LF: u8 = 0x6c;
const TAB: u8 = 0x74;
const SPACE: u8 = 0x73;
// const LF: u8 = 0x0A;
// const TAB: u8 = 0x9;
// const SPACE: u8 = 0x20;

pub struct WSParser {
    code: Vec<u8>,
    // code_index is the LAST READ character. NOT the next one to read
    code_index: usize,
    reader: io::BufReader<std::fs::File>,
}

impl Parser for WSParser {
    fn next_statement(&mut self) -> ParseResult<Statement> {
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

impl WSParser {
    pub fn new(reader: io::BufReader<std::fs::File>) -> Self {
        WSParser {
            code: vec![69],
            code_index: 0,
            reader,
        }
    }

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn index_char(&mut self) -> ParseResult<u8> {
        while self.code.len() <= self.code_index {
            let mut trans_buf = [0u8; 512];
            let nb_read = self.reader.read(&mut trans_buf)?;
            if nb_read == 0 {
                return Err(ParseError::EOF);
            }
            self.code.extend_from_slice(&trans_buf[..nb_read]);
        }
        Ok(self.code[self.code_index])
    }

    fn parse_io(&mut self) -> ParseResult<Statement> {
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
                        TAB => return Ok(Statement::PopStackOutputNumber),
                        SPACE => todo!(),
                        _ => (),
                    }
                },
                LF => return Err(ParseError::IO),
                _ => (),
            }
        }
    }

    fn parse_stack_manipulation(&mut self) -> ParseResult<Statement> {
        todo!()
    }

    fn parse_arithmetic(&mut self) -> ParseResult<Statement> {
        todo!()
    }

    fn parse_flow_control(&mut self) -> ParseResult<Statement> {
        todo!()
    }

    fn parse_head_access(&mut self) -> ParseResult<Statement> {
        todo!()
    }
}
