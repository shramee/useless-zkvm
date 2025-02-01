pub mod vm;
use crate::vm::run_vm;
use crate::vm::Op::{Add, Div, Mul, Push, Sub};

fn main() {
    let program = Vec::from([
        Push(3),
        Push(7),
        Add,
        Push(2),
        Mul,
        Push(3434),
        Div,
        Push(567),
        Sub,
    ]);

    run_vm(program.into());
}
