use std::io::{self, Read};

use crate::parser::{
    ParseErrorArithmetic, ParseErrorFlowCtrl, ParseErrorIO, ParseResult, ParseResultArithmetic, ParseResultFlowCtrl, ParseResultHeapAccess, ParseResultIO, ParseResultStackManip, Parser
};
use crate::statements::{Statement, StatementArithmetic, StatementFlowCtrl, StatementIO, StatementStackManip};

const LF: u8 = 0x6c;
const TAB: u8 = 0x74;
const SPACE: u8 = 0x73;
// const LF: u8 = 0x0A;
// const TAB: u8 = 0x9;
// const SPACE: u8 = 0x20;

pub struct WSParser {
    code: [u8; 2048],
    // code_index is the LAST READ character. NOT the next one to read
    code_length: usize,
    code_index: usize,
    reader: io::BufReader<std::fs::File>,
}

impl Parser for WSParser {
    fn next_statement(&mut self) -> ParseResult {
        loop {
            self.code_index += 1;
            match self.index_char()? {
                TAB => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        TAB => return Ok(Statement::HeapAccess(self.parse_head_access()?)),
                        SPACE => return Ok(Statement::Arithmetic(self.parse_arithmetic()?)),
                        LF => return Ok(Statement::IO(self.parse_io()?)),
                        _ => (),
                    }
                },
                SPACE => return Ok(Statement::StackManip(self.parse_stack_manipulation()?)),
                LF => return Ok(Statement::FlowCtrl(self.parse_flow_control()?)),
                _ => (),
            }
        }
    }
}

impl WSParser {
    pub fn new(reader: io::BufReader<std::fs::File>) -> Self {
        WSParser {
            code: [0; 2048],
            code_length: 0,
            code_index: 0,
            reader,
        }
    }

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn index_char(&mut self) -> io::Result<u8> {
        if self.code_length <= self.code_index {
            let nb_read = self.reader.read(&mut self.code)?;
            if nb_read == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
            }
            self.code_length = nb_read;
            self.code_index = 0;
        }
        Ok(self.code[self.code_index])
    }

    fn parse_io(&mut self) -> ParseResultIO {
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
                        TAB => return Ok(StatementIO::PopStackOutputNumber),
                        SPACE => return Ok(StatementIO::PopStackOutputChar),
                        _ => (),
                    }
                },
                LF => return Err(ParseErrorIO::ForbiddenLF),
                _ => (),
            }
        }
    }

    fn parse_stack_manipulation(&mut self) -> ParseResultStackManip {
        loop {
            self.code_index += 1;
            match self.index_char()? {
                SPACE => return Ok(StatementStackManip::Push(self.parse_number()?)),
                LF => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        SPACE => return Ok(StatementStackManip::DuplicateTopItem),
                        TAB => return Ok(StatementStackManip::SwapTopTwoItems),
                        LF => return Ok(StatementStackManip::DiscardTopItem),
                        _ => (),
                    }
                },
                TAB => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        SPACE => {
                            return Ok(StatementStackManip::CopyNthOnTop(self.parse_number()?));
                        }
                        LF => {
                            return Ok(StatementStackManip::SlideKeepTopItem(self.parse_number()?));
                        }
                        TAB => return Err(crate::parser::ParseErrorStackManip::ForbiddenTab),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn parse_arithmetic(&mut self) -> ParseResultArithmetic {
        loop {
            self.code_index += 1;
            
        match self.index_char()? {
            SPACE => loop {
                self.code_index += 1;
                match self.index_char()? {
                    SPACE => return Ok(StatementArithmetic::Addition),
                    TAB => return Ok(StatementArithmetic::Substraction),
                    LF => return Ok(StatementArithmetic::Multiplication),
                    _ => (),
                }
            },
            TAB => loop {
                self.code_index += 1;
                match self.index_char()? {
                    SPACE => return Ok(StatementArithmetic::IntegerDivision),
                    TAB => return Ok(StatementArithmetic::Modulo),
                    _ => (),
                }
            },
            LF => return Err(ParseErrorArithmetic::ForbiddenLF),
            _ => (),
        }
        }
    }

    fn parse_flow_control(&mut self) -> ParseResultFlowCtrl {
        loop {
            self.code_index += 1;
            match self.index_char()? {
                SPACE => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        SPACE => todo!(),
                        TAB => todo!(),
                        LF => todo!(),
                        _ => (),
                    }
                },
                TAB => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        SPACE => todo!(),
                        TAB => todo!(),
                        LF => return Err(ParseErrorFlowCtrl::ForbiddenLF),
                        _ => (),
                    }
                },
                LF => loop {
                    self.code_index += 1;
                    match self.index_char()? {
                        LF => return Ok(StatementFlowCtrl::EndProgram),
                        SPACE | TAB => return Err(ParseErrorFlowCtrl::WrongProgramEnd),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn parse_head_access(&mut self) -> ParseResultHeapAccess {
        todo!()
    }

    fn parse_number(&mut self) -> io::Result<i32> {
        let is_pos: bool;
        loop {
            self.code_index += 1;
            match self.index_char()? {
                SPACE => {
                    is_pos = true;
                    break;
                }
                TAB => {
                    is_pos = false;
                    break;
                }
                LF => return Ok(0),
                _ => (),
            }
        }

        let mut temp_int = 0;

        loop {
            self.code_index += 1;
            match self.index_char()? {
                SPACE => temp_int <<= 1,
                TAB => {
                    temp_int <<= 1;
                    temp_int |= 0b1
                }
                LF => break,
                _ => (),
            }
        }

        temp_int |= 0b0 << 31;
        if is_pos { Ok(temp_int) } else { Ok(-temp_int) }
    }
}
