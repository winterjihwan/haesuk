use crate::VMError;

const STACK_SIZE_LIMIT: usize = 1024;

type Word = usize;
pub struct VM {
    stack: [Word; STACK_SIZE_LIMIT],
    stack_size: usize,

    program: Program,
    program_size: usize,
    ip: usize,

    halt: bool,
}

#[derive(Debug, Clone)]
pub enum Inst {
    InstPush(Word),
    InstAdd,
    InstSub,
    InstMul,
    InstDiv,
    InstHalt,
    InstLoop(Word),
    InstEq(Word),
    InstDup(Word),
}

#[derive(Default)]
pub struct Program {
    pub insts: Vec<Inst>,
}

impl Inst {
    pub fn opcode(&self) -> u8 {
        match self {
            Inst::InstPush(_) => 0x01,
            Inst::InstAdd => 0x02,
            Inst::InstSub => 0x03,
            Inst::InstMul => 0x04,
            Inst::InstDiv => 0x05,
            Inst::InstHalt => 0x06,
            Inst::InstLoop(_) => 0x07,
            Inst::InstEq(_) => 0x08,
            Inst::InstDup(_) => 0x09,
        }
    }

    pub fn serialize<'a>(&self, bytes: &'a mut [u8; 16]) -> &'a [u8; 16] {
        bytes[0..16].copy_from_slice(&(self.opcode() as u128).to_le_bytes());

        bytes
    }

    pub fn serialize_operand<'a>(&self, bytes: &'a mut [u8; 16], operand: &Word) -> &'a [u8; 16] {
        bytes[0..8].copy_from_slice(&(self.opcode() as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&operand.to_le_bytes());

        bytes
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        match self {
            Inst::InstPush(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstAdd => *self.serialize(&mut bytes),
            Inst::InstSub => *self.serialize(&mut bytes),
            Inst::InstMul => *self.serialize(&mut bytes),
            Inst::InstDiv => *self.serialize(&mut bytes),
            Inst::InstHalt => *self.serialize(&mut bytes),
            Inst::InstLoop(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstEq(operand) => *self.serialize_operand(&mut bytes, operand),
            Inst::InstDup(operand) => *self.serialize_operand(&mut bytes, operand),
        }
    }
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.insts
            .iter()
            .for_each(|inst| bytes.extend(inst.to_bytes()));

        bytes
    }
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

    pub fn load_program(&mut self, program: Program) -> Result<(), VMError> {
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
                Inst::InstLoop(operand) => {
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
