use std::{fs::File, io::Read};

use crate::{inst::Inst, program::Program, word::Word, VMError};

const STACK_SIZE_LIMIT: usize = 1024;

#[derive(Debug)]
pub struct VM {
    stack: [Word; STACK_SIZE_LIMIT],
    stack_size: usize,

    program: Program,
    program_size: usize,
    ip: usize,

    halt: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: [Word::i64(0); 1024],
            stack_size: 0,

            program: Program::default(),
            program_size: 0,
            ip: 0,
            halt: false,
        }
    }

    #[deprecated]
    pub fn load_hasm_from_file(&mut self, path: &str) -> Result<(), VMError> {
        let mut file = File::open(path).map_err(|err| VMError::IoFail {
            err: err.to_string(),
        })?;

        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(|err| VMError::IoFail {
                err: err.to_string(),
            })?;

        let program = Program::from_hasm(&buffer)?;

        self.program_size = program.insts.len();
        self.program = program;

        Ok(())
    }

    pub fn load_ha_from_memory(&mut self, program: Program) -> Result<(), VMError> {
        self.program_size = program.insts.len();
        self.program = program;

        Ok(())
    }

    pub fn load_ha_from_file(&mut self, path: &str) -> Result<(), VMError> {
        let mut file = File::open(path).map_err(|err| VMError::IoFail {
            err: err.to_string(),
        })?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .map_err(|err| VMError::IoFail {
                err: err.to_string(),
            })?;

        let program = Program::from_bytes(&buffer)?;

        self.program_size = program.insts.len();
        self.program = program;

        Ok(())
    }

    pub fn run(&mut self, limit: Option<u16>) -> Result<(), VMError> {
        let limit = match limit {
            Some(l) => l,
            None => 64,
        };

        let mut loop_count: u16 = 0;
        while !self.halt && loop_count < limit {
            if self.ip >= self.program_size {
                return Err(VMError::SegmentFault);
            }
            let inst = &self.program.insts[self.ip];
            self.dump();

            match inst {
                Inst::InstPush(operand) => {
                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size] = *operand;
                    self.stack_size += 1;
                    self.ip += 1;
                }
                Inst::InstAddi => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }
                    self.stack[self.stack_size - 2] = Word::i64(
                        i64::from(self.stack[self.stack_size - 2])
                            + i64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstSubi => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] = Word::i64(
                        i64::from(self.stack[self.stack_size - 2])
                            - i64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstMuli => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] = Word::i64(
                        i64::from(self.stack[self.stack_size - 2])
                            * i64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstDivi => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    // Todo: 0 for all types?
                    if self.stack[self.stack_size - 2] == Word::u64(0) {
                        return Err(VMError::DivisionByZero);
                    }

                    self.stack[self.stack_size - 2] = Word::i64(
                        i64::from(self.stack[self.stack_size - 2])
                            / i64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstAddf => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }
                    self.stack[self.stack_size - 2] = Word::f64(
                        f64::from(self.stack[self.stack_size - 2])
                            + f64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstSubf => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] = Word::f64(
                        f64::from(self.stack[self.stack_size - 2])
                            - f64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstMulf => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] = Word::f64(
                        f64::from(self.stack[self.stack_size - 2])
                            * f64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstDivf => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    // Todo: 0 for all types?
                    if self.stack[self.stack_size - 2] == Word::u64(0) {
                        return Err(VMError::DivisionByZero);
                    }

                    self.stack[self.stack_size - 2] = Word::f64(
                        f64::from(self.stack[self.stack_size - 2])
                            / f64::from(self.stack[self.stack_size - 1]),
                    );
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstHalt => {
                    self.halt = true;
                }
                Inst::InstJmp(operand) => {
                    let n: u64 = (*operand).into();
                    self.ip = n as usize;
                }
                Inst::InstEq(operand) => {
                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size] = if self.stack[self.stack_size - 1] == *operand {
                        Word::u64(1)
                    } else {
                        Word::u64(0)
                    };
                    self.stack_size += 1;
                    self.ip += 1;
                }
                Inst::InstDup(operand) => {
                    let operand_u64 = u64::from(*operand);
                    if self.stack_size as u64 - operand_u64 <= 0 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size] =
                        self.stack[self.stack_size - 1 - operand_u64 as usize];
                    self.stack_size += 1;
                    self.ip += 1;
                }
                Inst::InstNop => {
                    self.ip += 1;
                }
            }
            loop_count += 1
        }

        Ok(())
    }

    pub fn dump(&self) {
        println!("Stack: ");
        (0..self.stack_size).for_each(|n| {
            let n = &self.stack[n];
            let n_u64: u64 = (*n).into();
            let n_i64: i64 = (*n).into();
            let n_f64: f64 = (*n).into();
            let n_ptr: *mut Word = (*n).into();
            println!(
                "\tu64: {}, i64: {}, f64: {}, ptr: {:?}",
                n_u64, n_i64, n_f64, n_ptr
            );
        })
    }
}
