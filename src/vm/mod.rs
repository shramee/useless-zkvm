enum Op {
    // Push value to Stack
    Push(u32),
    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,
}

pub struct VM {
    program: Vec<Op>,
}

impl Into<VM> for Vec<Op> {
    fn into(self) -> VM {
        VM { program: self }
    }
}

pub fn run_vm(program: VM) {
    program.program.iter().for_each(|op| match op {
        Op::Push(val) => println!("Pushing {}", val),
        Op::Add => println!("Adding"),
        Op::Sub => println!("Subtracting"),
        Op::Mul => println!("Multiplying"),
        Op::Div => println!("Dividing"),
    });
}
