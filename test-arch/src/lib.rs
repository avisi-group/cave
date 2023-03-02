use cave_core::ee::interp::{InstructionExecutor, Interpreter};
use decode::{TestArchDecoder, TestArchInstruction};
use disasm::TestArchDisassembler;

pub mod decode;
pub mod disasm;
pub mod interp;

#[derive(Debug)]
pub struct RegisterState {
    pub regs: [u64; 256],
    pub zf: bool,
}

impl RegisterState {
    pub fn new() -> Self {
        Self {
            regs: [0; 256],
            zf: true,
        }
    }
}

//pub type TestArchInterpreter = Interpreter<RegisterState, InstructionExecutor<RegisterState>>;
