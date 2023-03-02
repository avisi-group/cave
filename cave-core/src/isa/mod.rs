#[derive(Debug)]
pub enum DecodeError {
    UndefinedInstruction,
}

pub trait Decoder<I> {
    fn decode(&self, data: &[u8]) -> Result<(I, usize), DecodeError>;
}

#[derive(Debug)]
pub enum DisasmError {
    UnspecifiedError,
}

pub trait Disassembler<I> {
    fn disasm(&self, insn: &I) -> Result<String, DisasmError>;
}

pub trait Isa<I> {
    fn get_decoder(&self) -> &dyn Decoder<I>;
    fn get_disassembler(&self) -> &dyn Disassembler<I>;
}
