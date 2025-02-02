use itertools::Itertools;

use num_traits::{One, Zero};
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedBaseField;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use crate::vm::{Op, VM};

impl FrameworkEval for VM {
    fn log_size(&self) -> u32 {
        self.log_n_rows()
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows() + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut a = eval.next_trace_mask();
        let mut b = eval.next_trace_mask();
        self.program().iter().for_each(|op| {
            let c = eval.next_trace_mask();
            match op {
                Op::Push(_) => {
                    // nothing to do here
                }
                Op::Add => {
                    // c = a + b
                    eval.add_constraint(c.clone() - (a.clone() + b.clone()))
                }
                Op::Sub => {
                    // c = a - b
                    eval.add_constraint(c.clone() - (a.clone() - b.clone()))
                }
                Op::Mul => {
                    // c = a * b
                    eval.add_constraint(c.clone() - (a.clone() * b.clone()))
                }
                Op::Div => {
                    // division constraint is results * divisor = dividend
                    // a = c * b
                    eval.add_constraint(a.clone() - (c.clone() * b.clone()))
                }
            };
            // swapping to avoid cloning
            std::mem::swap(&mut a, &mut b);
            b = c;
        });
        eval
    }
}

pub fn generate_vm_trace(
    vm: &VM,
) -> ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
    let mut els: Vec<M31> = Vec::new();
    vm.program().into_iter().for_each(|op| {
        // col to modify
        let value = match op {
            Op::Push(value) => value.clone(),
            // op on the previous two elements
            Op::Add => els[els.len() - 1].clone() + els[els.len() - 2].clone(),
            Op::Sub => els[els.len() - 1].clone() - els[els.len() - 2].clone(),
            Op::Mul => els[els.len() - 1].clone() * els[els.len() - 2].clone(),
            Op::Div => els[els.len() - 1].clone() / els[els.len() - 2].clone(),
        };
        els.push(value);
    });

    let mut trace: Vec<BaseColumn> = Vec::new();
    let mut col = Col::<SimdBackend, BaseField>::

    while els.len() > 0 {
        col. = PackedBaseField::from_array(std::array::from_fn(|j| match els.pop() {
            Some(val) => val,
            None => BaseField::zero(),
        }));
        trace.push(col);
    }

    let domain = CanonicCoset::new(vm.log_size()).circle_domain();
    trace
        .into_iter()
        .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
        .collect_vec()
}
