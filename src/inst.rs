use crate::{VMError, Word};

#[derive(Debug, Clone)]
pub enum Inst {
    InstPush(Word),
    InstAdd,
    InstSub,
    InstMul,
    InstDiv,
    InstHalt,
    InstLoop(Word),
    InstEq(Word),
    InstDup(Word),
}

impl Inst {
    pub fn ser_opcode(&self) -> u8 {
        match self {
            Inst::InstPush(_) => 0x01,
            Inst::InstAdd => 0x02,
            Inst::InstSub => 0x03,
            Inst::InstMul => 0x04,
            Inst::InstDiv => 0x05,
            Inst::InstHalt => 0x06,
            Inst::InstLoop(_) => 0x07,
            Inst::InstEq(_) => 0x08,
            Inst::InstDup(_) => 0x09,
        }
    }

    pub fn deser_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            0x01 => Some(Inst::InstPush(0)),
            0x02 => Some(Inst::InstAdd),
            0x03 => Some(Inst::InstSub),
            0x04 => Some(Inst::InstMul),
            0x05 => Some(Inst::InstDiv),
            0x06 => Some(Inst::InstHalt),
            0x07 => Some(Inst::InstLoop(0)),
            0x08 => Some(Inst::InstEq(0)),
            0x09 => Some(Inst::InstDup(0)),
            _ => None,
        }
    }

    pub fn serialize<'a>(&self, bytes: &'a mut [u8; 16]) -> &'a [u8; 16] {
        bytes[0..16].copy_from_slice(&(self.ser_opcode() as u128).to_le_bytes());

        bytes
    }

    pub fn serialize_operand<'a>(&self, bytes: &'a mut [u8; 16], operand: &Word) -> &'a [u8; 16] {
        bytes[0..8].copy_from_slice(&(self.ser_opcode() as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&operand.to_le_bytes());

        bytes
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        match self {
            Inst::InstPush(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstAdd => *self.serialize(&mut bytes),
            Inst::InstSub => *self.serialize(&mut bytes),
            Inst::InstMul => *self.serialize(&mut bytes),
            Inst::InstDiv => *self.serialize(&mut bytes),
            Inst::InstHalt => *self.serialize(&mut bytes),
            Inst::InstLoop(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstEq(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstDup(operand) => *self.serialize_operand(&mut bytes, operand),
        }
    }

    fn with_operand(self, operand: usize) -> Self {
        match self {
            Inst::InstPush(_) => Inst::InstPush(operand),
            Inst::InstLoop(_) => Inst::InstLoop(operand),
            Inst::InstEq(_) => Inst::InstEq(operand),
            Inst::InstDup(_) => Inst::InstDup(operand),
            _ => self,
        }
    }

    pub fn from_bytes(bytes: &mut [u8; 16]) -> Result<Self, VMError> {
        let inst = Inst::deser_opcode(bytes[0]).ok_or(VMError::DeserializeOpcodeFail)?;

        let operand = usize::from_le_bytes(
            bytes[8..16]
                .try_into()
                .map_err(|_| VMError::ParseLeBytesFail)?,
        );

        let inst = inst.with_operand(operand);

        Ok(inst)
    }
}
