use stwo_prover::core::fields::m31::BaseField;

pub enum Op {
    // Push value to Stack
    Push(BaseField),
    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,
}

pub struct VM {
    _program: Vec<Op>,
    _log_n_rows: u32,
}

impl VM {
    pub fn new(program: Vec<Op>) -> Self {
        program.into()
    }

    pub fn program(&self) -> &Vec<Op> {
        &self._program
    }

    pub fn log_n_rows(&self) -> u32 {
        self._log_n_rows
    }
}

impl Into<VM> for Vec<Op> {
    fn into(self) -> VM {
        let rows = self.len();
        VM {
            _program: self,
            _log_n_rows: rows.next_power_of_two().trailing_zeros(),
        }
    }
}

pub fn run_vm(vm: VM) {
    vm._program.iter().for_each(|op| match op {
        Op::Push(val) => println!("Pushing {}", val),
        Op::Add => println!("Adding"),
        Op::Sub => println!("Subtracting"),
        Op::Mul => println!("Multiplying"),
        Op::Div => println!("Dividing"),
    });
}
