use stwo_prover::core::fields::m31::M31;

#[derive(Debug)]
pub enum Op {
    // Push value to Stack
    Push(M31),
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

    pub fn run(&self) -> M31 {
        let mut stack = Vec::new();
        self._program.iter().for_each(|op| {
            if let Op::Push(val) = op {
                stack.push(*val)
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if let Some(res) = match op {
                    Op::Add => Some(a + b),
                    Op::Sub => Some(a - b),
                    Op::Mul => Some(a * b),
                    Op::Div => Some(a / b),
                    _ => None,
                } {
                    stack.push(b);
                    stack.push(res);
                }
            }
        });

        return stack.pop().unwrap();
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
