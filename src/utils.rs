use crate::vm::{Op, VM};
use num_traits::Zero;
use stwo_prover::core::{
    backend::simd::{column::BaseColumn, m31::PackedBaseField},
    fields::m31::M31,
};
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

pub fn one_row_col(val: M31, length: usize) -> BaseColumn {
    return BaseColumn {
        data: vec![PackedBaseField::from_array(std::array::from_fn(|i| {
            if i == 0 {
                val
            } else {
                M31::zero()
            }
        }))],
        length,
    };
}
