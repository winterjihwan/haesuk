use std::{
    fs::File,
    io::{self, Read, Write},
};

use crate::{program::Program, VMError};

pub fn hasm_to_ha(path: &str) -> io::Result<()> {
    let mut file = File::open(path).map_err(|err| VMError::IoFail {
        err: err.to_string(),
    })?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .map_err(|err| VMError::IoFail {
            err: err.to_string(),
        })?;

    let program = Program::from_hasm(&buffer)?;
    let hasm_path = path.replace(".hasm", ".ha");

    let mut file = File::create(hasm_path)?;
    file.write_all(&program.to_bytes())?;

    Ok(())
}
