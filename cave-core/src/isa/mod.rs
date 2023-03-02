#[derive(Debug)]
pub enum DisasmError {
    UnspecifiedError,
}

pub trait Disassembler<I> {
    fn disasm(insn: &I) -> Result<String, DisasmError>;
}
