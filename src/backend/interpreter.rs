use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
};

use crate::{
    backend::runtime::{
        Runtime, RuntimeErrorArithmetic, RuntimeErrorFlowCtrl, RuntimeErrorHeap, RuntimeErrorIO,
        RuntimeErrorStackManip, RuntimeReport, RuntimeResult, RuntimeResultArithmetic,
        RuntimeResultFlowCtrl, RuntimeResultHeapAccess, RuntimeResultIO, RuntimeResultStackManip,
    },
    core::statements::{
        Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
        StatementStackManip,
    },
};

pub struct Interpreter {
    stack: Vec<i32>,
    heap: HashMap<i32, i32>,
    input: BufReader<Box<dyn Read>>,
    output: Box<dyn Write>,
}

impl Interpreter {
    pub fn new(input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
            input: BufReader::new(input),
            output,
        }
    }
}

impl Runtime for Interpreter {
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

impl Interpreter {
    fn run_io(&mut self, stat: StatementIO) -> RuntimeResultIO {
        use StatementIO::*;
        match stat {
            PopStackOutputNumber => self.pop_stack_output_number(),
            PopStackOutputChar => self.pop_stack_output_char(),
            ReadCharStoreOnHeap => self.read_char_store_on_heap(),
            ReadNumberStoreOnHeap => self.read_number_store_on_heap(),
        }
    }

    fn run_flow_ctrl(&mut self, stat: StatementFlowCtrl) -> RuntimeResultFlowCtrl {
        use StatementFlowCtrl::*;
        match stat {
            EndProgram => Ok(RuntimeReport::EndProgram),
            MarkLabel(label) => Ok(RuntimeReport::MarkLabel(label)),
            JumpTo(label) => Ok(RuntimeReport::JumpTo(label)),
            JumpToIfZero(label) => match self.stack.last() {
                Some(0) => Ok(RuntimeReport::JumpTo(label)),
                Some(..0 | 1..) => Ok(RuntimeReport::Next),
                None => Err(RuntimeErrorFlowCtrl::EmptyStack),
            },
            JumpToIfNegative(label) => match self.stack.last() {
                Some(..0) => Ok(RuntimeReport::JumpTo(label)),
                Some(0..) => Ok(RuntimeReport::Next),
                None => Err(RuntimeErrorFlowCtrl::EmptyStack),
            },
            CallSubroutine(label) => Ok(RuntimeReport::CallSubroutine(label)),
            ReturnFromSubroutine => Ok(RuntimeReport::ReturnFromSubroutine),
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
        let rhs = match self.stack.pop() {
            Some(x) => x,
            None => return Err(RuntimeErrorArithmetic::NoRhsOnStack),
        };
        let lhs = match self.stack.pop() {
            Some(x) => x,
            None => return Err(RuntimeErrorArithmetic::NoLhsOnStack),
        };

        use StatementArithmetic::*;
        let res = match stat {
            Addition => match lhs.checked_add(rhs) {
                Some(x) => x,
                None => return Err(RuntimeErrorArithmetic::UnderflowOrOverflow),
            },
            Substraction => match lhs.checked_sub(rhs) {
                Some(x) => x,
                None => return Err(RuntimeErrorArithmetic::UnderflowOrOverflow),
            },
            Multiplication => match lhs.checked_mul(rhs) {
                Some(x) => x,
                None => return Err(RuntimeErrorArithmetic::UnderflowOrOverflow),
            },
            IntegerDivision => match lhs.checked_div(rhs) {
                Some(x) => x,
                None => return Err(RuntimeErrorArithmetic::DivisionByZero),
            },
            Modulo => match lhs.checked_rem(rhs) {
                Some(x) => x,
                None => return Err(RuntimeErrorArithmetic::DivisionByZero),
            },
        };

        self.stack.push(res);
        Ok(RuntimeReport::Next)
    }

    fn run_heap_access(&mut self, stat: StatementHeapAccess) -> RuntimeResultHeapAccess {
        use StatementHeapAccess::*;
        match stat {
            Store => {
                let value = match self.stack.last() {
                    Some(val) => *val,
                    None => return Err(RuntimeErrorHeap::EmptyStack),
                };
                let location = match self.stack.get(self.stack.len() - 2) {
                    Some(val) => *val,
                    None => return Err(RuntimeErrorHeap::StackTooSmall),
                };
                self.heap.insert(location, value);
                Ok(RuntimeReport::Next)
            }
            Retrieve => {
                let location = match self.stack.last() {
                    Some(val) => *val,
                    None => return Err(RuntimeErrorHeap::StackTooSmall),
                };
                match self.heap.get(&location) {
                    Some(val) => self.stack.push(*val),
                    None => return Err(RuntimeErrorHeap::NothingAtAddress),
                }
                Ok(RuntimeReport::Next)
            }
        }
    }

    fn pop_stack_output_number(&mut self) -> RuntimeResultIO {
        // Should we pop the last element ?
        match self.stack.pop() {
            Some(i) => {
                write!(self.output, "{i}").unwrap();
                Ok(RuntimeReport::Next)
            }
            None => Err(RuntimeErrorIO::EmptyStack),
        }
    }

    fn pop_stack_output_char(&mut self) -> RuntimeResultIO {
        match self.stack.pop() {
            Some(i) => match char::from_u32(i as u32) {
                Some(c) => {
                    write!(self.output, "{c}").unwrap();
                    Ok(RuntimeReport::Next)
                }
                None => Err(RuntimeErrorIO::InvalidStoredUtf8Character),
            },
            None => Err(RuntimeErrorIO::EmptyStack),
        }
    }

    fn read_char_store_on_heap(&mut self) -> RuntimeResultIO {
        let address = match self.stack.pop() {
            Some(val) => val,
            None => return Err(RuntimeErrorIO::EmptyStack),
        };

        let mut buf = [0; 4];

        self.input.read_exact(&mut buf[0..1])?;

        let len = if buf[0] & 0x80 == 0 {
            1 // ASCII
        } else if buf[0] & 0xe0 == 0xc0 {
            2
        } else if buf[0] & 0xf0 == 0xe0 {
            3
        } else if buf[0] & 0xf8 == 0xf0 {
            4
        } else {
            return Err(RuntimeErrorIO::InvalidUtf8StartByte);
        };

        if len > 1 {
            self.input.read_exact(&mut buf[1..len])?
        }

        let s = match std::str::from_utf8(&buf[..len]) {
            Ok(s) => s,
            Err(e) => return Err(RuntimeErrorIO::ParseUtf8(e)),
        };

        let c = match s.chars().next() {
            Some(c) => c,
            None => panic!("Bro you literally just parsed a char wdym there's no char"),
        };

        self.heap.insert(address, c as i32);

        Ok(RuntimeReport::Next)
    }

    fn read_number_store_on_heap(&mut self) -> RuntimeResultIO {
        let address = match self.stack.pop() {
            Some(val) => val,
            None => return Err(RuntimeErrorIO::EmptyStack),
        };

        let mut s = String::new();
        self.input.read_line(&mut s)?;
        let number = s.trim().parse()?;

        self.heap.insert(address, number);

        Ok(RuntimeReport::Next)
    }

    fn push_on_stack(&mut self, val: i32) -> RuntimeResultStackManip {
        self.stack.push(val);
        Ok(RuntimeReport::Next)
    }

    fn duplicate_top_stack(&mut self) -> RuntimeResultStackManip {
        match self.stack.last() {
            Some(val) => {
                self.stack.push(*val);
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
