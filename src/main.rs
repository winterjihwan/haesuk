mod errors;
mod macros;
mod vm;

use std::{
    fs::File,
    io::{self, Write},
};

pub use errors::*;
pub use vm::*;

fn main() -> io::Result<()> {
    let mut vm = VM::new();

    let program = Program {
        insts: vec![
            Inst::InstPush(0),
            Inst::InstPush(1),
            Inst::InstDup(1),
            Inst::InstDup(1),
            Inst::InstAdd,
            Inst::InstLoop(2),
        ],
    };

    let mut file = File::create("fib.ha")?;
    file.write_all(&program.to_bytes())?;

    match vm.load_program(program) {
        Ok(_) => (),
        Err(err) => {
            println!("ERR:{}", err)
        }
    };

    match vm.run() {
        Ok(_) => (),
        Err(err) => {
            println!("ERR:{}", err)
        }
    };

    vm.dump();

    Ok(())
}
