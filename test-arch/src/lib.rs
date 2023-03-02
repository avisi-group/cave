use cave_core::isa::Isa;
use decode::{TestArchDecoder, TestArchInstruction};
use disasm::TestArchDisassembler;

mod decode;
mod disasm;
pub mod interp;

#[derive(Debug)]
pub struct CpuState {
    pub pc: u64,
    pub regs: [u64; 256],
    pub zf: bool,
    pub mem: *mut u8,
}

impl CpuState {
    pub fn new(mem: *mut u8) -> CpuState {
        CpuState {
            pc: 0,
            regs: [0; 256],
            zf: true,
            mem,
        }
    }
}

pub struct TestIsa {
    decoder: TestArchDecoder,
    disasm: TestArchDisassembler,
}

impl TestIsa {
    pub fn new() -> TestIsa {
        TestIsa {
            decoder: TestArchDecoder::default(),
            disasm: TestArchDisassembler::default(),
        }
    }
}

impl Isa<TestArchInstruction> for TestIsa {
    fn get_decoder(&self) -> &dyn cave_core::isa::Decoder<TestArchInstruction> {
        &self.decoder
    }

    fn get_disassembler(&self) -> &dyn cave_core::isa::Disassembler<TestArchInstruction> {
        &self.disasm
    }
}
