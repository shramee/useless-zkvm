use crate::vm::{Op, VM};
use stwo_prover::core::fields::m31::M31;
use Op::{Add, Div, Mul, Push, Sub};

pub fn felt(val: u32) -> M31 {
    M31::from_u32_unchecked(val)
}

pub fn dummy_program() -> VM {
    Vec::from([
        Push(felt(3)),
        Push(felt(7)),
        Add,
        Push(felt(6)),
        Mul,
        Push(felt(4)),
        Div,
        Push(felt(1)),
        Sub,
    ])
    .into()
}
