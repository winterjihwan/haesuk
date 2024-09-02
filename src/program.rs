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

    pub fn from_hasm(asm: &String) -> Result<Self, VMError> {
        let asm_insts: Vec<&str> = asm
            .split("\n")
            .filter(|inst| !inst.trim().is_empty())
            .collect();

        let insts = asm_insts
            .into_iter()
            .filter_map(|asm_inst| {
                let inst: Vec<&str> = asm_inst
                    .trim_start()
                    .split(" ")
                    .take_while(|elem| !elem.contains("#"))
                    .collect();

                if inst.is_empty() {
                    return None;
                }

                Some(match inst[0] {
                    "push" => Ok(Inst::InstPush(inst[1].parse::<usize>().unwrap())),
                    "add" => Ok(Inst::InstAdd),
                    "sub" => Ok(Inst::InstSub),
                    "mul" => Ok(Inst::InstMul),
                    "div" => Ok(Inst::InstDiv),
                    "halt" => Ok(Inst::InstHalt),
                    "jmp" => Ok(Inst::InstJmp(inst[1].parse::<usize>().unwrap())),
                    "eq" => Ok(Inst::InstEq(inst[1].parse::<usize>().unwrap())),
                    "dup" => Ok(Inst::InstDup(inst[1].parse::<usize>().unwrap())),
                    "#" => Ok(Inst::InstHalt),
                    _ => Err(VMError::InvalidAsmInst {
                        inst: inst[0].to_string(),
                    }),
                })
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        Ok(Self { insts })
    }

    pub fn to_hasm(&self) -> Vec<String> {
        self.insts
            .iter()
            .map(|inst| match inst {
                Inst::InstPush(operand) => format!("push {}", operand),
                Inst::InstAdd => "add".to_string(),
                Inst::InstSub => "sub".to_string(),
                Inst::InstMul => "mul".to_string(),
                Inst::InstDiv => "div".to_string(),
                Inst::InstHalt => "halt".to_string(),
                Inst::InstJmp(operand) => format!("jmp {}", operand),
                Inst::InstEq(operand) => format!("eq {}", operand),
                Inst::InstDup(operand) => format!("dup {}", operand),
            })
            .collect::<Vec<String>>()
    }
}
