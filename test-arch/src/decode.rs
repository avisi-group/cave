use byteorder::{LittleEndian, ByteOrder};
use cave_core::isa::{DecodeError, Decoder};

#[derive(Default)]
pub struct TestArchDecoder {}

pub enum TestArchInstruction {
    Nop,
    MovReg(u8, u8),
    MovImm(u8, u32),
    AddReg(u8, u8, u8),
    AddImm(u8, u8, u32),
    SubReg(u8, u8, u8),
    SubImm(u8, u8, u32),
    Jnz(i16),
    Hlt,
}

impl Decoder<TestArchInstruction> for TestArchDecoder {
    fn decode(
        &self,
        data: &[u8],
    ) -> Result<(TestArchInstruction, usize), cave_core::isa::DecodeError> {
        match data[0] {
            0 => Ok((TestArchInstruction::Nop, 1)),
            1 => Ok((TestArchInstruction::MovReg(data[1], data[2]), 3)),
            2 => Ok((TestArchInstruction::MovImm(data[1], LittleEndian::read_u32(&data[2..])), 6)),
            3 => Ok((TestArchInstruction::AddReg(data[1], data[2], data[3]), 4)),
            4 => Ok((TestArchInstruction::AddImm(data[1], data[2], LittleEndian::read_u32(&data[3..])), 7)),
            5 => Ok((TestArchInstruction::SubReg(data[1], data[2], data[3]), 4)),
            6 => Ok((TestArchInstruction::SubImm(data[1], data[2], LittleEndian::read_u32(&data[3..])), 7)),
            7 => Ok((TestArchInstruction::Jnz(LittleEndian::read_i16(&data[1..])), 3)),
            8 => Ok((TestArchInstruction::Hlt, 1)),
            _ => Err(DecodeError::UndefinedInstruction),
        }
    }
}
