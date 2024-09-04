use crate::{word::Word, VMError};

#[derive(Debug, Clone)]
pub enum Inst {
    InstPush(Word),
    InstAddi,
    InstSubi,
    InstMuli,
    InstDivi,

    InstAddf,
    InstSubf,
    InstMulf,
    InstDivf,

    InstHalt,
    InstJmp(Word),
    InstEq(Word),
    InstDup(Word),
    InstNop,
}

impl Inst {
    pub fn ser_opcode(&self) -> u8 {
        match self {
            Inst::InstPush(_) => 0x01,

            Inst::InstAddi => 0x02,
            Inst::InstSubi => 0x03,
            Inst::InstMuli => 0x04,
            Inst::InstDivi => 0x05,

            Inst::InstAddf => 0x06,
            Inst::InstSubf => 0x07,
            Inst::InstMulf => 0x08,
            Inst::InstDivf => 0x09,

            Inst::InstHalt => 0x0A,
            Inst::InstJmp(_) => 0x0B,
            Inst::InstEq(_) => 0x0C,
            Inst::InstDup(_) => 0x0D,
            Inst::InstNop => 0x0E,
        }
    }

    pub fn deser_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            0x01 => Some(Inst::InstPush(Word::u64(0))),

            0x02 => Some(Inst::InstAddi),
            0x03 => Some(Inst::InstSubi),
            0x04 => Some(Inst::InstMuli),
            0x05 => Some(Inst::InstDivi),

            0x06 => Some(Inst::InstAddf),
            0x07 => Some(Inst::InstSubf),
            0x08 => Some(Inst::InstMulf),
            0x09 => Some(Inst::InstDivf),

            0x0A => Some(Inst::InstHalt),
            0x0B => Some(Inst::InstJmp(Word::u64(0))),
            0x0C => Some(Inst::InstEq(Word::u64(0))),
            0x0D => Some(Inst::InstDup(Word::u64(0))),
            0x0E => Some(Inst::InstNop),
            _ => None,
        }
    }

    pub fn serialize<'a>(&self, bytes: &'a mut [u8; 16]) -> &'a [u8; 16] {
        bytes[0..16].copy_from_slice(&(self.ser_opcode() as u128).to_le_bytes());

        bytes
    }

    pub fn serialize_operand<'a>(&self, bytes: &'a mut [u8; 16], operand: &Word) -> &'a [u8; 16] {
        bytes[0..8].copy_from_slice(&(self.ser_opcode() as usize).to_le_bytes());
        bytes[8..16].copy_from_slice(&operand.to_le_bytes());

        bytes
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        match self {
            Inst::InstPush(operand) => *self.serialize_operand(&mut bytes, operand),

            Inst::InstAddi
            | Inst::InstSubi
            | Inst::InstMuli
            | Inst::InstDivi
            | Inst::InstAddf
            | Inst::InstSubf
            | Inst::InstMulf
            | Inst::InstDivf => *self.serialize(&mut bytes),

            Inst::InstHalt => *self.serialize(&mut bytes),
            Inst::InstJmp(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstEq(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstDup(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstNop => *self.serialize(&mut bytes),
        }
    }

    fn with_operand(self, op_bytes: &mut [u8; 8]) -> Self {
        match self {
            Inst::InstPush(_) => Inst::InstPush(Word::from_le_bytes::<i64>(*op_bytes)),
            Inst::InstJmp(_) => Inst::InstJmp(Word::from_le_bytes::<u64>(*op_bytes)),
            Inst::InstEq(_) => Inst::InstEq(Word::from_le_bytes::<u64>(*op_bytes)),
            Inst::InstDup(_) => Inst::InstDup(Word::from_le_bytes::<u64>(*op_bytes)),
            _ => self,
        }
    }

    pub fn from_bytes(bytes: &mut [u8; 16]) -> Result<Self, VMError> {
        let inst = Inst::deser_opcode(bytes[0]).ok_or(VMError::DeserializeOpcodeFail)?;

        let mut op_bytes: [u8; 8] = bytes[8..16].try_into().unwrap();
        let inst = inst.with_operand(&mut op_bytes);

        Ok(inst)
    }
}
