use std::{fs::File, io::Read};

use crate::{inst::Inst, program::Program, VMError};

const STACK_SIZE_LIMIT: usize = 1024;

pub type Word = usize;

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
            stack: [0; 1024],
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

    pub fn run(&mut self) -> Result<(), VMError> {
        let mut loop_count = 0;
        while !self.halt && loop_count < 60 {
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
                Inst::InstAdd => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] += self.stack[self.stack_size - 1];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstSub => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] -= self.stack[self.stack_size - 1];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstMul => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size - 2] *= self.stack[self.stack_size - 1];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstDiv => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    if self.stack[self.stack_size - 2] == 0 {
                        return Err(VMError::DivisionByZero);
                    }

                    self.stack[self.stack_size - 2] /= self.stack[self.stack_size - 1];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                Inst::InstHalt => {
                    self.halt = true;
                }
                Inst::InstJmp(operand) => {
                    self.ip = *operand;
                }
                Inst::InstEq(operand) => {
                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size] = if self.stack[self.stack_size - 1] == *operand {
                        1
                    } else {
                        0
                    };
                    self.stack_size += 1;
                    self.ip += 1;
                }
                Inst::InstDup(operand) => {
                    if self.stack_size - operand <= 0 {
                        return Err(VMError::StackUnderflow { inst: inst.clone() });
                    }

                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow { inst: inst.clone() });
                    }

                    self.stack[self.stack_size] = self.stack[self.stack_size - 1 - operand];
                    self.stack_size += 1;
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
            println!("\t{}", self.stack[n]);
        })
    }
}
