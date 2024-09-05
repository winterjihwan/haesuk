use std::{
    fs::File,
    io::{self, Read, Write},
};

use crate::{program::Program, word::Word, VMError};

pub fn ha_to_hasm(path: &str) -> io::Result<()> {
    let mut file = File::open(path).map_err(|err| VMError::IoFail {
        err: err.to_string(),
    })?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .map_err(|err| VMError::IoFail {
            err: err.to_string(),
        })?;

    let program = Program::from_bytes(&buffer)?;
    let hasm = program.to_hasm();

    let hasm_path = path.replace(".ha", ".hasm");

    let mut file = File::create(hasm_path)?;
    for inst in hasm {
        writeln!(file, "{}", inst)?;
    }

    Ok(())
}

pub fn hasm_with_operand(hasm: String, operand: Word) -> String {
    format!("{} {}", hasm, operand)
}
