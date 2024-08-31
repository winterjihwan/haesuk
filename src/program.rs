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
}
