pub mod vm;
use crate::vm::run_vm;
use crate::vm::Op::{Add, Div, Mul, Push, Sub};

pub fn felt(val: u32) -> BaseField {
    BaseField::from_u32_unchecked(val)
}

fn main() {
    let program = Vec::from([
        Push(felt(3)),
        Push(felt(7)),
        Add,
        Push(felt(2)),
        Mul,
        Push(felt(3434)),
        Div,
        Push(felt(567)),
        Sub,
    ]);

    run_vm(program.into());
}
