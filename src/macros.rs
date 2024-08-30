#[macro_export]
macro_rules! inst {
    ($i_type:expr, $operand:expr) => {
        Inst {
            i_type: $i_type,
            operand: Some($operand),
        }
    };
    ($i_type:expr) => {
        Inst {
            i_type: $i_type,
            operand: None,
        }
    };
}

//#[macro_export]
//macro_rules! insts {
//    ($($i_type:expr, $operand:expr);* $(;)?) => {
//        vec![
//        $(inst!($i_type, $operand), )*
//        ]
//    };
//}
