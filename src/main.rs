mod errors;
mod inst;
mod macros;
mod program;
mod vm;

use std::{
    fs::File,
    io::{self, Write},
};

pub use errors::*;
use program::Program;
pub use vm::*;

fn save_program_to_file(program: &Program, path: &str) -> io::Result<()> {
    //let program = Program {
    //    insts: vec![
    //        Inst::InstPush(0),
    //        Inst::InstPush(1),
    //        Inst::InstDup(1),
    //        Inst::InstDup(1),
    //        Inst::InstAdd,
    //        Inst::InstLoop(2),
    //    ],
    //};

    //save_program_to_file(&program, "fib.ha")?;

    let mut file = File::create(path)?;
    file.write_all(&program.to_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut vm = VM::new();

    let fib_path = "fib.ha";
    vm.load_program_from_file(fib_path).unwrap();

    //vm.load_program_from_memory(program)
    //    .map_err(io::Error::from)?;

    vm.run().map_err(io::Error::from)?;

    vm.dump();

    Ok(())
}
