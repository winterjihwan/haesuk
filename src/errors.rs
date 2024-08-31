use thiserror::Error;

use crate::Inst;

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
}
