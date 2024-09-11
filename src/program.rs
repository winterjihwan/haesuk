use std::{collections::HashMap, ops::Deref, process::exit, str::FromStr, u16};

use crate::{
    dehasm::hasm_with_operand,
    inst::{Inst, INST_TRANSLATE, OPERAND_REQUIRED},
    VMError,
};

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
    pub deferred_operands: HMCache<u16, String>,
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

                let interpret_hasm =
                    |inst: Vec<&str>, tc: &mut TranslationContext, program_size_t: &mut u16| {
                        //
                        let inst_str = *INST_TRANSLATE.extract_key(&inst[0]);
                        let maybe_operand = inst.get(1).map(Deref::deref);
                        let inst: Inst = Inst::from_str(inst_str).unwrap();
                        let inst = inst.resolve_operand(maybe_operand, tc, program_size_t);
                        *program_size_t += 1;
                        Ok(inst)
                    };

                if inst.first()?.ends_with(":") {
                    let label = inst.first()?.replace(":", "");
                    assert!(tc.label_table.cache_size + 1 < LABLE_TABLE_CAPACITY);
                    tc.label_table.hash_map.insert(label, program_size_t);
                    tc.label_table.cache_size += 1;

                    let possible_inst = inst[1..].to_vec();
                    if !possible_inst.is_empty() {
                        return Some(interpret_hasm(possible_inst, &mut tc, &mut program_size_t));
                    }

                    return None;
                }

                Some(interpret_hasm(inst, &mut tc, &mut program_size_t))
            })
            .collect::<Result<Vec<Inst>, VMError>>()?;

        tc.deferred_operands
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
            .map(|inst| {
                let asm_inst = (*INST_TRANSLATE.extract_val(&inst.as_ref())).to_string();

                if *OPERAND_REQUIRED.get(inst.as_ref()).unwrap_or(&false) {
                    return match inst {
                        Inst::InstPush(operand)
                        | Inst::InstDup(operand)
                        | Inst::InstEq(operand)
                        | Inst::InstJmp(operand) => hasm_with_operand(asm_inst, *operand),
                        _ => exit(2),
                    };
                }

                asm_inst
            })
            .collect::<Vec<String>>()
    }
}
