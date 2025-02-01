enum Op {
    // Push value to Stack
    Push,
    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,
}

struct VM {
    instructions: Vec<Op>,
    stack: Vec<u32>,
}
