use std::io::Write;

use crate::{
    runtime::{
        Runtime, RuntimeErrorIO, RuntimeErrorStackManip, RuntimeReport, RuntimeResult,
        RuntimeResultArithmetic, RuntimeResultFlowCtrl, RuntimeResultHeapAccess, RuntimeResultIO,
        RuntimeResultStackManip,
    },
    statements::{
        Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
        StatementStackManip,
    },
};

pub struct DirectRuntime<O: Write> {
    stack: Vec<i32>,
    pub output: O,
}

impl<Output: Write> DirectRuntime<Output> {
    pub fn new(output: Output) -> Self {
        Self {
            stack: Vec::new(),
            output,
        }
    }
}

impl<O: Write> Runtime for DirectRuntime<O> {
    fn run_statement(&mut self, statement: Statement) -> RuntimeResult {
        use Statement::*;
        match statement {
            IO(stat_io) => Ok(self.run_io(stat_io)?),
            FlowCtrl(stat_fwcl) => Ok(self.run_flow_ctrl(stat_fwcl)?),
            StackManip(stat_skmp) => Ok(self.run_stack_manip(stat_skmp)?),
            Arithmetic(stat_ac) => Ok(self.run_arithmetic(stat_ac)?),
            HeapAccess(stat_hpas) => Ok(self.run_heap_access(stat_hpas)?),
        }
    }
}

impl<O: Write> DirectRuntime<O> {
    fn run_io(&mut self, stat: StatementIO) -> RuntimeResultIO {
        use StatementIO::*;
        match stat {
            PopStackOutputNumber => self.pop_stack_output_number(),
            PopStackOutputChar => self.pop_stack_output_char(),
        }
    }

    fn run_flow_ctrl(&mut self, stat: StatementFlowCtrl) -> RuntimeResultFlowCtrl {
        use StatementFlowCtrl::*;
        match stat {
            EndProgram => Ok(RuntimeReport::EndProgram),
        }
    }

    fn run_stack_manip(&mut self, stat: StatementStackManip) -> RuntimeResultStackManip {
        use StatementStackManip::*;
        match stat {
            Push(i) => self.push_on_stack(i),
            DuplicateTopItem => self.duplicate_top_stack(),
            SwapTopTwoItems => self.swap_top_two_stack(),
            DiscardTopItem => self.discard_top_stack(),
            CopyNthOnTop(i) => self.copy_nth_top_stack(i),
            SlideKeepTopItem(i) => self.slide_keep_top_stack(i),
        }
    }

    fn run_arithmetic(&mut self, stat: StatementArithmetic) -> RuntimeResultArithmetic {
        todo!()
    }

    fn run_heap_access(&mut self, stat: StatementHeapAccess) -> RuntimeResultHeapAccess {
        todo!()
    }

    fn pop_stack_output_number(&mut self) -> RuntimeResultIO {
        // Should we pop the last element ?
        match self.stack.pop() {
            Some(i) => {
                write!(self.output, "{i}").unwrap();
                Ok(RuntimeReport::Next)
            }
            None => Err(RuntimeErrorIO::ReadEmptyStack),
        }
    }

    fn pop_stack_output_char(&mut self) -> RuntimeResultIO {
        match self.stack.pop() {
            Some(i) => match char::from_u32(i as u32) {
                Some(c) => {
                    write!(self.output, "{c}").unwrap();
                    Ok(RuntimeReport::Next)
                }
                None => Err(RuntimeErrorIO::InvalidUTF8Character),
            },
            None => Err(RuntimeErrorIO::ReadEmptyStack),
        }
    }

    fn push_on_stack(&mut self, i: i32) -> RuntimeResultStackManip {
        self.stack.push(i);
        Ok(RuntimeReport::Next)
    }

    fn duplicate_top_stack(&mut self) -> RuntimeResultStackManip {
        match self.stack.last_mut() {
            Some(i) => {
                *i *= 2;
                Ok(RuntimeReport::Next)
            }
            None => Err(RuntimeErrorStackManip::EmptyStack),
        }
    }

    fn swap_top_two_stack(&mut self) -> RuntimeResultStackManip {
        if self.stack.len() < 2 {
            return Err(RuntimeErrorStackManip::StackTooSmall);
        }

        let len = self.stack.len();
        (self.stack[len - 1], self.stack[len - 2]) = (self.stack[len - 2], self.stack[len - 1]);
        Ok(RuntimeReport::Next)
    }

    fn discard_top_stack(&mut self) -> RuntimeResultStackManip {
        match self.stack.pop() {
            Some(_) => Ok(RuntimeReport::Next),
            None => Err(RuntimeErrorStackManip::EmptyStack),
        }
    }

    fn copy_nth_top_stack(&mut self, i: i32) -> RuntimeResultStackManip {
        let i = i as usize;

        if i < 1 || i > self.stack.len() {
            return Err(RuntimeErrorStackManip::NotInStackRange);
        }

        self.stack.push(self.stack[i - 1]);
        Ok(RuntimeReport::Next)
    }

    fn slide_keep_top_stack(&mut self, i: i32) -> RuntimeResultStackManip {
        let last = match self.stack.last() {
            Some(n) => *n,
            None => return Err(RuntimeErrorStackManip::EmptyStack),
        };
        let i = std::cmp::min(i as usize, self.stack.len() - 1);

        self.stack.truncate(self.stack.len() - i - 1);
        self.stack.push(last);
        Ok(RuntimeReport::Next)
    }
}
