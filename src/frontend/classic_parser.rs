use std::io::{self, Read};

use crate::core::statements::{
    Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
    StatementStackManip,
};
use crate::frontend::parser::{
    ParseErrorArithmetic, ParseErrorFlowCtrl, ParseErrorHeapAccess, ParseErrorIO,
    ParseErrorStackManip, ParseResult, ParseResultArithmetic, ParseResultFlowCtrl,
    ParseResultHeapAccess, ParseResultIO, ParseResultStackManip, Parser, TokenKind,
};

#[derive(Copy, Clone, Debug)]
pub enum ParsedLanguage {
    WrittenWhitespace,
    ClassicWhitespace { lf: u8, tab: u8, space: u8 },
}

use TokenKind::*;

pub struct ClassicParser {
    // code_index is the LAST READ character. NOT the next one to read
    code: Box<[u8; 4194304]>,
    code_length: usize,
    code_index: usize,
    reader: Box<dyn Read>,
    language: ParsedLanguage,
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
    pub fn new(reader: Box<dyn Read>, language: ParsedLanguage) -> Self {
        ClassicParser {
            code: Box::new([0; _]),
            code_length: 0,
            code_index: 0,
            reader,
            language,
        }
    }

    // WARN This does not support UTF-8 at all. There are possibilities to do very very ugly things..........
    fn next_char(&mut self) -> io::Result<u8> {
        self.code_index += 1;
        if self.code_length <= self.code_index {
            let nb_read = self.reader.read(&mut *self.code)?;
            if nb_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "reached end of file",
                ));
            }
            self.code_length = nb_read;
            self.code_index = 0;
        }
        Ok(self.code[self.code_index])
    }

    fn next_token(&mut self) -> Result<TokenKind, io::Error> {
        use ParsedLanguage::*;
        match self.language {
            WrittenWhitespace => self.next_wws_token(),
            ClassicWhitespace { lf, tab, space } => self.next_classic_token(lf, tab, space),
        }
    }

    fn next_wws_token(&mut self) -> Result<TokenKind, io::Error> {
        loop {
            if self.next_char()? != b'[' {
                continue;
            }
            let mut token_val = [0; 6];
            for i  in 0..6 {
                token_val[i] = self.next_char()?;
                if &token_val[..3] == b"LF]" {
                    return Ok(TokenKind::Lf);
                } else if &token_val[..4] == b"Tab]" {
                    return Ok(TokenKind::Tab);
                } else if &token_val[..6] == b"Space]" {
                    return Ok(TokenKind::Space);
                }
            }
            return Err(io::Error::new(io::ErrorKind::InvalidData, "error while parsing token"))

        }
    }

    fn next_classic_token(&mut self, lf: u8, tab: u8, space: u8) -> Result<TokenKind, io::Error> {
        loop {
            let next_char = self.next_char()?;
            if next_char == lf {
                return Ok(TokenKind::Lf);
            } else if next_char == tab {
                return Ok(TokenKind::Tab);
            } else if next_char == space {
                return Ok(TokenKind::Space);
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
                Space => Ok(StatementFlowCtrl::MarkLabel(self.parse_number()?)),
                Tab => Ok(StatementFlowCtrl::CallSubroutine(self.parse_number()?)),
                Lf => Ok(StatementFlowCtrl::JumpTo(self.parse_number()?)),
            },
            Tab => match self.next_token()? {
                Space => Ok(StatementFlowCtrl::JumpToIfZero(self.parse_number()?)),
                Tab => Ok(StatementFlowCtrl::JumpToIfNegative(self.parse_number()?)),
                Lf => Ok(StatementFlowCtrl::ReturnFromSubroutine),
            },
            Lf => match self.next_token()? {
                Lf => Ok(StatementFlowCtrl::EndProgram),
                Space | Tab => Err(ParseErrorFlowCtrl::WrongProgramEnd),
            },
        }
    }

    fn parse_heap_access(&mut self) -> ParseResultHeapAccess {
        match self.next_token()? {
            Space => Ok(StatementHeapAccess::Store),
            Tab => Ok(StatementHeapAccess::Retrieve),
            Lf => Err(ParseErrorHeapAccess::ForbiddenLF),
        }
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
