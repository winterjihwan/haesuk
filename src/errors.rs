use thiserror::Error;

use crate::InstType;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Stack overflow while operating on {inst:?}")]
    StackOverflow { inst: InstType },

    #[error("Stack underflow while operating on {inst:?}")]
    StackUnderflow { inst: InstType },

    #[error("Operand non exists while operating on {inst:?}")]
    OperandNonExists { inst: InstType },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Segment fault")]
    SegmentFault,

    #[error("Invalid operand")]
    InvalidOperand,
}
