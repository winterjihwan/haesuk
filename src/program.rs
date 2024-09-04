use std::{collections::HashMap, u16};

use crate::{inst::Inst, word::Word, VMError};

pub const LABLE_TABLE_CAPACITY: u16 = u16::MAX;
pub const DEFERRED_OPERANDS_CAPACITY: u16 = u16::MAX;

#[derive(Default, Debug)]
pub struct Program {
    pub insts: Vec<Inst>,
}

#[derive(Default, Debug)]
pub struct HMCache<K, V> {
    pub hash_map: HashMap<K, V>,
    pub cache_size: u16,
}

#[derive(Default, Debug)]
pub struct TranslationContext {
    pub label_table: HMCache<String, u16>,
    pub defered_operands: HMCache<u16, String>,
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
                // ["push", "3", "", "#", "why", "not", "push", "4", ""]
                // ["push", "3", ""]
                // ["push", "3"]
                let inst: Vec<&str> = asm_inst
                    .trim_start()
                    .split(" ")
                    .take_while(|elem| !elem.contains("#"))
                    .filter(|s| !s.is_empty())
                    .collect();

                if inst.is_empty() {
                    return None;
                }

                let mut interpret_hasm = |inst: Vec<&str>, program_size_t: &mut u16| {
                    let inst = match inst[0] {
                        "push" => Ok(Inst::InstPush(inst[1].parse::<u64>().unwrap().into())),
                        "add" => Ok(Inst::InstAddi),
                        "sub" => Ok(Inst::InstSubi),
                        "mul" => Ok(Inst::InstMuli),
                        "div" => Ok(Inst::InstDivi),
                        "halt" => Ok(Inst::InstHalt),
                        "jmp" => {
                            let operand = inst[1];
                            if operand.chars().next().unwrap().is_numeric() {
                                Ok(Inst::InstJmp(operand.parse::<u64>().unwrap().into()))
                            } else {
                                assert!(
                                    tc.defered_operands.cache_size + 1 < DEFERRED_OPERANDS_CAPACITY
                                );
                                tc.defered_operands
                                    .hash_map
                                    .insert(*program_size_t, operand.to_string());
                                tc.defered_operands.cache_size += 1;
                                Ok(Inst::InstJmp(Word::u64(0)))
                            }
                        }
                        "eq" => Ok(Inst::InstEq(inst[1].parse::<u64>().unwrap().into())),
                        "dup" => Ok(Inst::InstDup(inst[1].parse::<u64>().unwrap().into())),
                        "nop" => Ok(Inst::InstNop),
                        "#" => Ok(Inst::InstHalt),
                        _ => Err(VMError::InvalidAsmInst {
                            inst: inst[0].to_string(),
                        }),
                    };
                    *program_size_t += 1;
                    inst
                };

                // loop: dup 2
                // ["loop:", "dup", "2"]
                // ["dup:", "2"]
                if inst.first()?.ends_with(":") {
                    let label = inst.first()?.replace(":", "");
                    assert!(tc.label_table.cache_size + 1 < LABLE_TABLE_CAPACITY);
                    tc.label_table.hash_map.insert(label, program_size_t);
                    tc.label_table.cache_size += 1;

                    let possible_inst = inst[1..].to_vec();
                    if !possible_inst.is_empty() {
                        return Some(interpret_hasm(possible_inst, &mut program_size_t));
                    }

                    return None;
                }

                Some(interpret_hasm(inst, &mut program_size_t))
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        tc.defered_operands
            .hash_map
            .into_iter()
            .try_for_each(|(inst_index, label)| {
                if let Inst::InstJmp(_) = &mut insts[inst_index as usize] {
                    let resolved_label = tc
                        .label_table
                        .hash_map
                        .get(&label)
                        .ok_or(VMError::ResolveLabelFail)?;
                    insts[inst_index as usize] = Inst::InstJmp(((*resolved_label) as u64).into())
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

                Inst::InstAddi => "addi".to_string(),
                Inst::InstSubi => "subi".to_string(),
                Inst::InstMuli => "muli".to_string(),
                Inst::InstDivi => "divi".to_string(),

                Inst::InstAddf => "addf".to_string(),
                Inst::InstSubf => "subf".to_string(),
                Inst::InstMulf => "mulf".to_string(),
                Inst::InstDivf => "divf".to_string(),

                Inst::InstHalt => "halt".to_string(),
                Inst::InstJmp(operand) => format!("jmp {}", operand),
                Inst::InstEq(operand) => format!("eq {}", operand),
                Inst::InstDup(operand) => format!("dup {}", operand),
                Inst::InstNop => "nop".to_string(),
            })
            .collect::<Vec<String>>()
    }
}
