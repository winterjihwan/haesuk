mod errors;
mod macros;

use errors::VMError;
use std::io;

const STACK_SIZE_LIMIT: usize = 1024;

type Word = isize;
struct VM {
    stack: [Word; STACK_SIZE_LIMIT],
    stack_size: usize,

    program: Program,
    program_size: usize,
    ip: usize,
}

#[derive(Debug, Clone)]
enum InstType {
    InstPush,
    InstAdd,
    InstSub,
    InstMul,
    InstDiv,

    InstLoop,
}

struct Inst {
    i_type: InstType,
    operand: Option<isize>,
}

#[derive(Default)]
struct Program {
    insts: Vec<Inst>,
}

impl VM {
    fn new() -> Self {
        Self {
            stack: [0; 1024],
            stack_size: 0,

            program: Program::default(),
            program_size: 0,
            ip: 0,
        }
    }

    fn load_program(&mut self, program: Program) -> Result<(), VMError> {
        self.program_size = program.insts.len();
        self.program = program;

        Ok(())
    }

    fn run(&mut self) -> Result<(), VMError> {
        for _ in 0..15 {
            if self.ip >= self.program_size {
                return Err(VMError::SegmentFault);
            }
            let inst = &self.program.insts[self.ip];
            self.dump();

            match inst.i_type {
                InstType::InstPush => {
                    if self.stack_size >= STACK_SIZE_LIMIT {
                        return Err(VMError::StackOverflow {
                            inst: inst.i_type.clone(),
                        });
                    }

                    let operand = inst.operand.ok_or(VMError::OperandNonExists {
                        inst: inst.i_type.clone(),
                    })?;

                    self.stack[self.stack_size] = operand;
                    self.stack_size += 1;
                    self.ip += 1;
                }
                InstType::InstAdd => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow {
                            inst: inst.i_type.clone(),
                        });
                    }

                    self.stack[self.stack_size - 2] =
                        self.stack[self.stack_size - 1] + self.stack[self.stack_size - 2];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                InstType::InstSub => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow {
                            inst: inst.i_type.clone(),
                        });
                    }

                    self.stack[self.stack_size - 2] =
                        self.stack[self.stack_size - 1] - self.stack[self.stack_size - 2];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                InstType::InstMul => {
                    if self.stack_size <= 2 {
                        return Err(VMError::StackUnderflow {
                            inst: inst.i_type.clone(),
                        });
                    }

                    self.stack[self.stack_size - 2] =
                        self.stack[self.stack_size - 1] * self.stack[self.stack_size - 2];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                InstType::InstDiv => {
                    if self.stack_size < 2 {
                        return Err(VMError::StackUnderflow {
                            inst: inst.i_type.clone(),
                        });
                    }

                    if self.stack[self.stack_size - 2] == 0 {
                        return Err(VMError::DivisionByZero);
                    }

                    self.stack[self.stack_size - 2] =
                        self.stack[self.stack_size - 1] / self.stack[self.stack_size - 2];
                    self.stack_size -= 1;
                    self.ip += 1;
                }
                InstType::InstLoop => {
                    let operand = inst.operand.ok_or(VMError::OperandNonExists {
                        inst: inst.i_type.clone(),
                    })?;

                    self.ip = operand.try_into().map_err(|_| VMError::InvalidOperand)?;
                }
            }

            if self.ip == self.program_size {
                break;
            }
        }

        Ok(())
    }

    fn dump(&self) {
        println!("Stack: ");
        (0..self.stack_size).for_each(|n| {
            println!("\t{}", self.stack[n]);
        })
    }
}

fn main() -> io::Result<()> {
    let mut vm = VM::new();

    let program = Program {
        insts: vec![
            inst!(InstType::InstPush, 5),
            inst!(InstType::InstPush, 13),
            inst!(InstType::InstPush, 7),
            inst!(InstType::InstAdd),
            inst!(InstType::InstPush, 2),
            inst!(InstType::InstLoop, 0),
        ],
    };

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
