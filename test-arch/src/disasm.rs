use cave_core::isa::{DisasmError, Disassembler};

use crate::decode::TestArchInstruction;

#[derive(Default)]
pub struct TestArchDisassembler {}

impl Disassembler<TestArchInstruction> for TestArchDisassembler {
    fn disasm(&self, insn: &TestArchInstruction) -> Result<String, cave_core::isa::DisasmError> {
        match insn {
            TestArchInstruction::Nop => Ok(String::from("nop")),
            TestArchInstruction::Hlt => Ok(String::from("hlt")),
            TestArchInstruction::MovImm(dst, val) => {
                Ok(String::from(format!("mov %{}, ${}", dst, val)))
            }
            TestArchInstruction::MovReg(dst, src) => {
                Ok(String::from(format!("mov %{}, %{}", dst, src)))
            }
            TestArchInstruction::AddReg(dst, src1, src2) => {
                Ok(String::from(format!("add %{}, %{}, %{}", dst, src1, src2)))
            }
            TestArchInstruction::SubImm(dst, src1, src2) => {
                Ok(String::from(format!("sub %{}, %{}, ${}", dst, src1, src2)))
            }
            TestArchInstruction::Jnz(offset) => Ok(String::from(format!("jnz {}", offset))),
            _ => Err(DisasmError::UnspecifiedError),
        }
    }
}
