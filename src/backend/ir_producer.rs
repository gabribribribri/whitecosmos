use std::io::{BufWriter, Write};

use crate::{
    backend::runtime::{Runtime, RuntimeError, RuntimeReport},
    core::statements::{Statement, StatementFlowCtrl},
};

pub struct IrProducer {
    writer: BufWriter<Box<dyn Write>>,
}

impl IrProducer {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }
}

impl Runtime for IrProducer {
    fn run_statement(&mut self, statement: Statement) -> Result<RuntimeReport, RuntimeError> {
        match wincode::serialize_into(&mut self.writer, &statement) {
            Ok(_) => (),
            Err(err) => return Err(RuntimeError::IrWriteError(err)),
        }

        match statement {
            Statement::FlowCtrl(StatementFlowCtrl::EndProgram) => {
                self.writer.flush();
                Ok(RuntimeReport::EndProgram)
            }
            _ => Ok(RuntimeReport::Next),
        }
    }
}
