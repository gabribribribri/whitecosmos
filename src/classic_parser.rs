use std::io::{self, Read};

use crate::parser::{
    ParseErrorArithmetic, ParseErrorFlowCtrl, ParseErrorIO, ParseErrorStackManip, ParseResult,
    ParseResultArithmetic, ParseResultFlowCtrl, ParseResultHeapAccess, ParseResultIO,
    ParseResultStackManip, Parser,
};
use crate::statements::{
    Statement, StatementArithmetic, StatementFlowCtrl, StatementIO, StatementStackManip,
};

#[derive(Copy, Clone, Debug)]
pub struct TokenValues {
    pub lf: u8,
    pub tab: u8,
    pub space: u8,
}

enum Token {
    Lf,
    Tab,
    Space,
}

use Token::*;

pub struct ClassicParser {
    // code_index is the LAST READ character. NOT the next one to read
    code: [u8; 2048],
    code_length: usize,
    code_index: usize,
    reader: Box<dyn Read>,
    tokens: TokenValues,
}

impl Parser for ClassicParser {
    fn next_statement(&mut self) -> ParseResult {
        match self.next_token()? {
            Tab => match self.next_token()? {
                Tab => Ok(Statement::HeapAccess(self.parse_heap_access()?)),
                Space => Ok(Statement::Arithmetic(self.parse_arithmetic()?)),
                Lf => Ok(Statement::IO(self.parse_io()?)),
            },
            Space => Ok(Statement::StackManip(self.parse_stack_manipulation()?)),
            Lf => Ok(Statement::FlowCtrl(self.parse_flow_control()?)),
        }
    }
}

impl ClassicParser {
    pub fn new(reader: Box<dyn Read>, tokens: TokenValues) -> Self {
        ClassicParser {
            code: [0; 2048],
            code_length: 0,
            code_index: 0,
            reader,
            tokens,
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

    fn next_token(&mut self) -> Result<Token, io::Error> {
        loop {
            self.code_index += 1;
            let next_char = self.index_char()?;
            if next_char == self.tokens.lf {
                return Ok(Token::Lf);
            } else if next_char == self.tokens.tab {
                return Ok(Token::Tab);
            } else if next_char == self.tokens.space {
                return Ok(Token::Space);
            }
        }
    }

    fn parse_io(&mut self) -> ParseResultIO {
        match self.next_token()? {
            Space => match self.next_token()? {
                Tab => Ok(StatementIO::PopStackOutputNumber),
                Space => Ok(StatementIO::PopStackOutputChar),
                Lf => Err(ParseErrorIO::ForbiddenLF),
            },
            Tab => match self.next_token()? {
                Tab => todo!(),
                Space => todo!(),
                Lf => Err(ParseErrorIO::ForbiddenLF),
            },
            Lf => Err(ParseErrorIO::ForbiddenLF),
        }
    }

    fn parse_stack_manipulation(&mut self) -> ParseResultStackManip {
        match self.next_token()? {
            Space => Ok(StatementStackManip::Push(self.parse_number()?)),
            Lf => match self.next_token()? {
                Space => Ok(StatementStackManip::DuplicateTopItem),
                Tab => Ok(StatementStackManip::SwapTopTwoItems),
                Lf => Ok(StatementStackManip::DiscardTopItem),
            },
            Tab => match self.next_token()? {
                Space => Ok(StatementStackManip::CopyNthOnTop(self.parse_number()?)),
                Lf => Ok(StatementStackManip::SlideKeepTopItem(self.parse_number()?)),
                Tab => Err(ParseErrorStackManip::ForbiddenTab),
            },
        }
    }

    fn parse_arithmetic(&mut self) -> ParseResultArithmetic {
        match self.next_token()? {
            Space => match self.next_token()? {
                Space => Ok(StatementArithmetic::Addition),
                Tab => Ok(StatementArithmetic::Substraction),
                Lf => Ok(StatementArithmetic::Multiplication),
            },
            Tab => match self.next_token()? {
                Space => Ok(StatementArithmetic::IntegerDivision),
                Tab => Ok(StatementArithmetic::Modulo),
                Lf => Err(ParseErrorArithmetic::ForbiddenLF),
            },
            Lf => Err(ParseErrorArithmetic::ForbiddenLF),
        }
    }

    fn parse_flow_control(&mut self) -> ParseResultFlowCtrl {
        match self.next_token()? {
            Space => match self.next_token()? {
                Space => todo!(),
                Tab => todo!(),
                Lf => todo!(),
            },
            Tab => match self.next_token()? {
                Space => todo!(),
                Tab => todo!(),
                Lf => Err(ParseErrorFlowCtrl::ForbiddenLF),
            },
            Lf => match self.next_token()? {
                Lf => Ok(StatementFlowCtrl::EndProgram),
                Space | Tab => Err(ParseErrorFlowCtrl::WrongProgramEnd),
            },
        }
    }

    fn parse_heap_access(&mut self) -> ParseResultHeapAccess {
        todo!()
    }

    fn parse_number(&mut self) -> io::Result<i32> {
        let is_pos: bool;
        match self.next_token()? {
            Space => is_pos = true,
            Tab => is_pos = false,
            Lf => return Ok(0),
        }

        let mut temp_int = 0;

        loop {
            match self.next_token()? {
                Space => temp_int <<= 1,
                Tab => {
                    temp_int <<= 1;
                    temp_int |= 0b1;
                }
                Lf => break,
            }
        }

        temp_int |= 0b0 << 31;
        if is_pos { Ok(temp_int) } else { Ok(-temp_int) }
    }
}
