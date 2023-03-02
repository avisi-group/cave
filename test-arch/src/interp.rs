use std::num::Wrapping;

use cave_core::ee::interp::{ExecutionResult, InstructionExecutor};
use cave_core::ee::CpuState;

use crate::decode::{TestArchDecoder, TestArchInstruction};
use crate::disasm::TestArchDisassembler;
use crate::RegisterState;

pub struct TestArchInstructionExecutor {}

impl InstructionExecutor<RegisterState> for TestArchInstructionExecutor {
    fn execute(state: &mut CpuState<RegisterState>) -> ExecutionResult {
        let (insn, len) = TestArchDecoder::decode(unsafe {
            std::slice::from_raw_parts(state.mem.offset(state.pc as isize), 16)
        })
        .unwrap();

        println!("{}", TestArchDisassembler::disasm(&insn).unwrap());

        match insn {
            TestArchInstruction::MovImm(dst, val) => {
                state.regstate.regs[dst as usize] = val as u64;
                state.pc += len;
                ExecutionResult::Continue
            }
            TestArchInstruction::MovReg(dst, src) => {
                state.regstate.regs[dst as usize] = state.regstate.regs[src as usize];
                state.pc += len;
                ExecutionResult::Continue
            }
            TestArchInstruction::AddReg(dst, src1, src2) => {
                state.regstate.regs[dst as usize] = (Wrapping(state.regstate.regs[src1 as usize])
                    + Wrapping(state.regstate.regs[src2 as usize]))
                .0;
                state.regstate.zf = state.regstate.regs[dst as usize] == 0;
                state.pc += len;
                ExecutionResult::Continue
            }
            TestArchInstruction::SubImm(dst, src1, src2) => {
                state.regstate.regs[dst as usize] =
                    (Wrapping(state.regstate.regs[src1 as usize]) - Wrapping(src2 as u64)).0;

                state.regstate.zf = state.regstate.regs[dst as usize] == 0;
                state.pc += len;
                ExecutionResult::Continue
            }
            TestArchInstruction::Jnz(offset) => {
                if !state.regstate.zf {
                    state.pc = (state.pc as i64 + offset as i64) as usize;
                } else {
                    state.pc += len;
                }
                ExecutionResult::Continue
            }
            TestArchInstruction::Hlt => ExecutionResult::Exit,
            _ => ExecutionResult::Abort,
        }
    }
}
