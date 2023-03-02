use std::num::Wrapping;

use cave_core::ee::interp::{Interpreter, StepError, StepMode, StepResult};
use cave_core::isa::{self, Disassembler};

use crate::{
    decode::{TestArchDecoder, TestArchInstruction},
    disasm::TestArchDisassembler,
    CpuState,
};
use cave_core::isa::Decoder;

pub struct TestArchInterpreter<'a> {
    cpu_state: &'a mut CpuState,
    decoder: TestArchDecoder,
    disassembler: TestArchDisassembler,
}

impl<'a> TestArchInterpreter<'a> {
    pub fn new(cpu_state: &'a mut CpuState) -> TestArchInterpreter<'a> {
        TestArchInterpreter {
            cpu_state,
            decoder: TestArchDecoder::default(),
            disassembler: TestArchDisassembler::default(),
        }
    }

    fn execute(
        &mut self,
        (insn, len): (TestArchInstruction, usize),
    ) -> Result<StepResult, StepError> {
        //println!("{}", self.disassembler.disasm(&insn).unwrap());

        match insn {
            TestArchInstruction::MovImm(dst, val) => {
                self.cpu_state.regs[dst as usize] = val as u64;
                self.cpu_state.pc += len as u64;
                Ok(StepResult::Ok(1))
            }
            TestArchInstruction::MovReg(dst, src) => {
                self.cpu_state.regs[dst as usize] = self.cpu_state.regs[src as usize];
                self.cpu_state.pc += len as u64;
                Ok(StepResult::Ok(1))
            }
            TestArchInstruction::AddReg(dst, src1, src2) => {
                self.cpu_state.regs[dst as usize] = (Wrapping(self.cpu_state.regs[src1 as usize])
                    + Wrapping(self.cpu_state.regs[src2 as usize]))
                .0;
                self.cpu_state.zf = self.cpu_state.regs[dst as usize] == 0;
                self.cpu_state.pc += len as u64;
                Ok(StepResult::Ok(1))
            }
            TestArchInstruction::SubImm(dst, src1, src2) => {
                self.cpu_state.regs[dst as usize] =
                    (Wrapping(self.cpu_state.regs[src1 as usize]) - Wrapping(src2 as u64)).0;

                self.cpu_state.zf = self.cpu_state.regs[dst as usize] == 0;
                self.cpu_state.pc += len as u64;
                Ok(StepResult::Ok(1))
            }
            TestArchInstruction::Jnz(offset) => {
                if !self.cpu_state.zf {
                    self.cpu_state.pc = (self.cpu_state.pc as i64 + offset as i64) as u64;
                } else {
                    self.cpu_state.pc += len as u64;
                }
                Ok(StepResult::Ok(1))
            }
            TestArchInstruction::Hlt => Ok(StepResult::Exited(1)),
            _ => Err(StepError::DecodeError),
        }
    }
}

impl<'a> Interpreter for TestArchInterpreter<'a> {
    fn step(&mut self, mode: StepMode) -> Result<StepResult, StepError> {
        let mut icount = 0;

        match mode {
            StepMode::MaxInstructions(c) => {
                while icount < c {
                    let data = unsafe {
                        std::slice::from_raw_parts(
                            self.cpu_state.mem.offset(self.cpu_state.pc as isize),
                            16,
                        )
                    };
                    let i = self.decoder.decode(data).unwrap();

                    match self.execute(i)? {
                        StepResult::Ok(n) => {
                            icount += n;
                        }
                        StepResult::Exited(n) => {
                            return Ok(StepResult::Exited(icount + n));
                        }
                    };
                }

                Ok(StepResult::Ok(icount))
            }
            StepMode::Forever => loop {
                let data = unsafe {
                    std::slice::from_raw_parts(
                        self.cpu_state.mem.offset(self.cpu_state.pc as isize),
                        16,
                    )
                };
                let i = self.decoder.decode(data).unwrap();

                match self.execute(i)? {
                    StepResult::Ok(n) => {
                        icount += n;
                    }
                    StepResult::Exited(n) => {
                        return Ok(StepResult::Exited(icount + n));
                    }
                };
            },
            _ => Err(StepError::DecodeError),
        }
    }
}
