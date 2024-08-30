type Word = usize;
struct VM {
    stack: [Word; 1024],
    stack_size: usize,
}

enum InstType {
    InstPush,
    InstAdd,
}

struct Inst {
    r#type: InstType,
    operand: Option<usize>,
}

fn main() {}
