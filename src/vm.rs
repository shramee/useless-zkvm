use stwo_prover::core::fields::m31::{BaseField, M31};

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

    pub fn run(&self) -> M31 {
        let mut stack = Vec::new();
        self._program.iter().for_each(|op| match op {
            Op::Push(val) => stack.push(*val),
            Op::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = a + b;
                stack.push(b);
                stack.push(res);
            }
            Op::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = a - b;
                stack.push(b);
                stack.push(res);
            }
            Op::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = a * b;
                stack.push(b);
                stack.push(res);
            }
            Op::Div => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = a / b;
                stack.push(b);
                stack.push(res);
            }
        });
        println!("Result: {:?}", stack.pop().unwrap());

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
