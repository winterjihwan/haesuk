use crate::{inst::Inst, VMError};

#[derive(Default, Debug)]
pub struct Program {
    pub insts: Vec<Inst>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.insts
            .iter()
            .for_each(|inst| bytes.extend(inst.to_bytes()));

        bytes
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, VMError> {
        assert!(bytes.len() % 16 == 0);

        let insts = bytes
            .chunks_exact(16)
            .map(|chunk| {
                let mut inst_bytes: [u8; 16] =
                    chunk.try_into().map_err(|_| VMError::ParseLeBytesFail)?;
                Ok(Inst::from_bytes(&mut inst_bytes).map_err(|_| VMError::ParseLeBytesFail)?)
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        Ok(Self { insts })
    }

    pub fn from_hasm(asm: &String) -> Result<Program, VMError> {
        let asm_insts: Vec<&str> = asm
            .split("\n")
            .filter(|inst| !inst.trim().is_empty())
            .collect();

        let insts = asm_insts
            .into_iter()
            .map(|asm_inst| {
                let elems: Vec<&str> = asm_inst.split(" ").collect();
                match elems[0] {
                    "push" => Ok(Inst::InstPush(elems[1].parse::<usize>().unwrap())),
                    "add" => Ok(Inst::InstAdd),
                    "sub" => Ok(Inst::InstSub),
                    "mul" => Ok(Inst::InstMul),
                    "div" => Ok(Inst::InstDiv),
                    "halt" => Ok(Inst::InstHalt),
                    "loop" => Ok(Inst::InstLoop(elems[1].parse::<usize>().unwrap())),
                    "eq" => Ok(Inst::InstEq(elems[1].parse::<usize>().unwrap())),
                    "dup" => Ok(Inst::InstDup(elems[1].parse::<usize>().unwrap())),
                    _ => Err(VMError::InvalidAsmInst {
                        inst: elems[0].to_string(),
                    }),
                }
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        Ok(Self { insts })
    }
}
