use std::collections::HashMap;

use lazy_static::lazy_static;
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{
    bimap::Bimap,
    program::{TranslationContext, DEFERRED_OPERANDS_CAPACITY},
    word::Word,
    VMError,
};

#[derive(Debug, Clone, Display, PartialEq, AsRefStr, EnumString)]
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

lazy_static! {
    pub static ref INST_TRANSLATE: Bimap<&'static str, &'static str> = {
        let mut bimap = Bimap::default();
        bimap.insert(Inst::InstPush(Word::u64(0)).as_ref(), "push");
        bimap.insert(Inst::InstAddi.as_ref(), "addi");
        bimap.insert(Inst::InstSubi.as_ref(), "subi");
        bimap.insert(Inst::InstMuli.as_ref(), "muli");
        bimap.insert(Inst::InstDivi.as_ref(), "divi");
        bimap.insert(Inst::InstAddf.as_ref(), "addf");
        bimap.insert(Inst::InstSubf.as_ref(), "subf");
        bimap.insert(Inst::InstMulf.as_ref(), "mulf");
        bimap.insert(Inst::InstDivf.as_ref(), "divf");
        bimap.insert(Inst::InstHalt.as_ref(), "halt");
        bimap.insert(Inst::InstJmp(Word::u64(0)).as_ref(), "jmp");
        bimap.insert(Inst::InstEq(Word::u64(0)).as_ref(), "eq");
        bimap.insert(Inst::InstDup(Word::u64(0)).as_ref(), "dup");
        bimap.insert(Inst::InstNop.as_ref(), "nop");
        bimap
    };
}

lazy_static! {
    pub static ref OPERAND_REQUIRED: HashMap<&'static str, bool> = {
        let mut map = HashMap::new();
        map.insert(Inst::InstPush(Word::u64(0)).as_ref(), true);
        map.insert(Inst::InstJmp(Word::u64(0)).as_ref(), true);
        map.insert(Inst::InstEq(Word::u64(0)).as_ref(), true);
        map.insert(Inst::InstDup(Word::u64(0)).as_ref(), true);

        map
    };
}

impl Inst {
    pub fn translate(&self) -> &str {
        ""
    }
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

    pub fn with_operand_word(
        self,
        maybe_operand_str: Option<&&str>,
        tc: &mut TranslationContext,
        program_size_t: &mut u16,
    ) -> Self {
        let operand_str = maybe_operand_str.unwrap();

        match self {
            Inst::InstPush(_) => Inst::InstPush(Word::i64(operand_str.parse::<i64>().unwrap())),
            Inst::InstJmp(_) => {
                if (operand_str).chars().next().unwrap().is_numeric() {
                    Inst::InstJmp((operand_str).parse::<u64>().unwrap().into())
                } else {
                    assert!(tc.defered_operands.cache_size + 1 < DEFERRED_OPERANDS_CAPACITY);
                    tc.defered_operands
                        .hash_map
                        .insert(*program_size_t, (operand_str).to_string());
                    tc.defered_operands.cache_size += 1;
                    Inst::InstJmp(Word::u64(0))
                };

                Inst::InstJmp(Word::i64(operand_str.parse::<i64>().unwrap()))
            }
            Inst::InstEq(_) => Inst::InstEq(Word::u64(operand_str.parse::<u64>().unwrap())),
            Inst::InstDup(_) => Inst::InstDup(Word::u64(operand_str.parse::<u64>().unwrap())),
            _ => self,
        }
    }

    pub fn with_operand_bytes(self, op_bytes: &mut [u8; 8]) -> Self {
        match self {
            Inst::InstPush(_) => Inst::InstPush(Word::from_le_bytes::<i64>(*op_bytes)),
            Inst::InstJmp(_) => Inst::InstJmp(Word::from_le_bytes::<u64>(*op_bytes)),
            Inst::InstEq(_) => Inst::InstEq(Word::from_le_bytes::<u64>(*op_bytes)),
            Inst::InstDup(_) => Inst::InstDup(Word::from_le_bytes::<u64>(*op_bytes)),
            _ => self,
        }
    }

    pub fn resolve_operand(
        self,
        maybe_operand_str: Option<&&str>,
        tc: &mut TranslationContext,
        program_size_t: &mut u16,
    ) -> Self {
        if *OPERAND_REQUIRED.get(self.as_ref()).unwrap_or(&false) {
            return self.with_operand_word(maybe_operand_str, tc, program_size_t);
        }
        self
    }

    pub fn from_bytes(bytes: &mut [u8; 16]) -> Result<Self, VMError> {
        let inst = Inst::deser_opcode(bytes[0]).ok_or(VMError::DeserializeOpcodeFail)?;

        let mut op_bytes: [u8; 8] = bytes[8..16].try_into().unwrap();
        let inst = inst.with_operand_bytes(&mut op_bytes);

        Ok(inst)
    }
}
