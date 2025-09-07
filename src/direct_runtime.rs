use crate::{
    runtime::{
        Runtime, RuntimeErrorIO, RuntimeReport, RuntimeResult,
        RuntimeResultArithmetic, RuntimeResultFlowCtrl, RuntimeResultHeapAccess, RuntimeResultIO,
        RuntimeResultStackManip,
    },
    statements::{
        Statement, StatementArithmetic, StatementFlowCtrl, StatementHeapAccess, StatementIO,
        StatementStackManip,
    },
};

pub struct DirectRuntime {
    stack: Vec<i32>,
}

impl DirectRuntime {
    pub fn new() -> Self {
        Self {
            stack: vec![1, 2, 3, 4242],
            // stack: Vec::new(),
        }
    }
}

impl Runtime for DirectRuntime {
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

impl DirectRuntime {
    fn run_io(&mut self, stat: StatementIO) -> RuntimeResultIO {
        use StatementIO::*;
        match stat {
            PopStackOutputNumber => self.pop_stack_output_number(),
        }
    }

    fn run_flow_ctrl(&mut self, stat: StatementFlowCtrl) -> RuntimeResultFlowCtrl {
        use StatementFlowCtrl::*;
        match stat {
            EndProgram => Ok(RuntimeReport::EndProgram),
        }
    }

    fn run_stack_manip(&mut self, stat: StatementStackManip) -> RuntimeResultStackManip {
        todo!()
    }

    fn run_arithmetic(&mut self, stat: StatementArithmetic) -> RuntimeResultArithmetic {
        todo!()
    }

    fn run_heap_access(&mut self, stat: StatementHeapAccess) -> RuntimeResultHeapAccess {
        todo!()
    }

    fn pop_stack_output_number(&mut self) -> Result<RuntimeReport, RuntimeErrorIO> {
        // Should we pop the last element ?
        match self.stack.pop() {
            Some(i) => {
                print!("{i}");
                Ok(RuntimeReport::Next)
            }
            None => Err(RuntimeErrorIO::ReadEmptyStack),
        }
    }
}
