use std::{collections::HashMap, u16};

use crate::{inst::Inst, VMError, Word};

const LABLE_TABLE_CAPACITY: u16 = u16::MAX;
const UNRESOLVED_JUMPS_CAPACITY: u16 = u16::MAX;

#[derive(Default, Debug)]
pub struct Program {
    pub insts: Vec<Inst>,
}

#[derive(Default, Debug)]
struct HMCache<K, V> {
    pub hash_map: HashMap<K, V>,
    pub cache_size: u16,
}

#[derive(Default, Debug)]
struct TranslationContext<'a> {
    label_table: HMCache<String, u16>,
    unresolved_jmps: HMCache<&'a str, u16>,
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
        let mut tc = TranslationContext::default();
        let mut program_size_t: u16 = 0;

        let asm_insts: Vec<&str> = asm
            .split("\n")
            .filter(|inst| !inst.trim().is_empty())
            .collect();

        let mut insts = asm_insts
            .into_iter()
            .filter_map(|asm_inst| {
                // \tpush 3 # why not push 4?
                // push 3 # why not push 4?
                // ["push", "3", "#", "why", "not", "push", "4"]
                // ["push", "3"]
                let inst: Vec<&str> = asm_inst
                    .trim_start()
                    .split(" ")
                    .take_while(|elem| !elem.contains("#"))
                    .collect();

                if inst.is_empty() {
                    return None;
                }

                if inst.first()?.ends_with(":") {
                    let label = inst.first()?.replace(":", "");
                    assert!(tc.label_table.cache_size + 1 < LABLE_TABLE_CAPACITY);
                    tc.label_table.hash_map.insert(label, program_size_t);

                    return None;
                }

                Some({
                    let inst = match inst[0] {
                        "push" => Ok(Inst::InstPush(inst[1].parse::<Word>().unwrap())),
                        "add" => Ok(Inst::InstAdd),
                        "sub" => Ok(Inst::InstSub),
                        "mul" => Ok(Inst::InstMul),
                        "div" => Ok(Inst::InstDiv),
                        "halt" => Ok(Inst::InstHalt),
                        "jmp" => {
                            let operand = inst[1];
                            assert!(tc.unresolved_jmps.cache_size + 1 < UNRESOLVED_JUMPS_CAPACITY);
                            tc.unresolved_jmps.hash_map.insert(operand, program_size_t);
                            Ok(Inst::InstJmp(0))
                        }
                        "eq" => Ok(Inst::InstEq(inst[1].parse::<Word>().unwrap())),
                        "dup" => Ok(Inst::InstDup(inst[1].parse::<Word>().unwrap())),
                        "#" => Ok(Inst::InstHalt),
                        _ => Err(VMError::InvalidAsmInst {
                            inst: inst[0].to_string(),
                        }),
                    };
                    program_size_t += 1;
                    inst
                })
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        tc.unresolved_jmps
            .hash_map
            .into_iter()
            .try_for_each(|(label, inst_index)| {
                if let Inst::InstJmp(_) = &mut insts[inst_index as usize] {
                    let resolved_label = tc
                        .label_table
                        .hash_map
                        .get(label)
                        .ok_or(VMError::ResolveLabelFail)?;
                    insts[inst_index as usize] = Inst::InstJmp((*resolved_label).into())
                }

                Ok(())
            })?;

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
