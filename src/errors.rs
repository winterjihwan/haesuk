use std::io;

use thiserror::Error;

use crate::inst::Inst;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Stack overflow while operating on {inst:?}")]
    StackOverflow { inst: Inst },

    #[error("Stack underflow while operating on {inst:?}")]
    StackUnderflow { inst: Inst },

    #[error("Operand non exists while operating on {inst:?}")]
    OperandNonExists { inst: Inst },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Segment fault")]
    SegmentFault,

    #[error("Invalid operand")]
    InvalidOperand,

    #[error("Deserialize opcode failed")]
    DeserializeOpcodeFail,

    #[error("Parse Le bytes fail")]
    ParseLeBytesFail,

    #[error("Invalid asm inst, inst: {inst}")]
    InvalidAsmInst { inst: String },

    #[error("I/O fail, err: {err}")]
    IoFail { err: String },

    #[error("Resolve label fail")]
    ResolveLabelFail,
}

impl From<VMError> for io::Error {
    fn from(error: VMError) -> Self {
        io::Error::new(io::ErrorKind::Other, format!("{:#?}", error))
    }
}
