//#[macro_export]
//macro_rules! inst {
//    (InstPush, $operand:expr) => {
//        Inst::InstPush($operand)
//    };
//    (InstAdd) => {
//        Inst::InstAdd
//    };
//    (InstSub) => {
//        Inst::InstSub
//    };
//    (InstMul) => {
//        Inst::InstMul
//    };
//    (InstDiv) => {
//        Inst::InstDiv
//    };
//    (InstHalt) => {
//        Inst::InstHalt
//    };
//    (InstLoop, $operand:expr) => {
//        Inst::InstLoop($operand)
//    };
//    (InstEq, $operand:expr) => {
//        Inst::InstEq($operand)
//    };
//    (InstDup, $operand:expr) => {
//        Inst::InstDup($operand)
//    };
//}

//#[macro_export]
//macro_rules! insts {
//    ($($i_type:expr, $operand:expr);* $(;)?) => {
//        vec![
//        $(inst!($i_type, $operand), )*
//        ]
//    };
//}
