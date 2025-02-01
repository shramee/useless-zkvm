pub enum Op {
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
    pub log_n_rows: u32,
    // stack: Vec<Op>,
}

impl Into<VM> for Vec<Op> {
    fn into(self) -> VM {
        let rows = self.len();
        VM {
            program: self,
            log_n_rows: rows.next_power_of_two().trailing_zeros(),
        }
    }
}

pub fn run_vm(vm: VM) {
    vm.program.iter().for_each(|op| match op {
        Op::Push(val) => println!("Pushing {}", val),
        Op::Add => println!("Adding"),
        Op::Sub => println!("Subtracting"),
        Op::Mul => println!("Multiplying"),
        Op::Div => println!("Dividing"),
    });
}
