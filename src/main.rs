pub mod vm;
use crate::vm::run_vm;
use crate::vm::Op::{Add, Div, Mul, Push, Sub};
use stwo_prover::core::fields::m31::BaseField;

pub fn felt(val: u32) -> BaseField {
    BaseField::from_u32_unchecked(val)
}

fn main() {
    let program = Vec::from([
        Push(felt(3)),
        Push(felt(7)),
        Add,
        Push(felt(6)),
        Mul,
        Push(felt(4)),
        Div,
        Push(felt(1)),
        Sub,
    ]);

    let result = run_vm(program.into());
    println!("Result: {}", result);
}
