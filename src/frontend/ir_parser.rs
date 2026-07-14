use std::io::{BufReader, Read};

use crate::{
    core::statements::Statement,
    frontend::parser::{ParseError, Parser},
};

pub struct IrParser {
    reader: BufReader<Box<dyn Read>>,
}

impl IrParser {
    pub fn new(reader: Box<dyn Read>) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }
}

impl Parser for IrParser {
    fn next_statement(&mut self) -> Result<Statement, ParseError> {
        match wincode::deserialize_from::<Statement>(&mut self.reader) {
            Ok(st) => Ok(st),
            Err(err) => Err(ParseError::IrReadError(err)),
        }
    }
}
