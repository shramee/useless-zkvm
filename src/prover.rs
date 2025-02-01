use itertools::Itertools;

use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::backend::simd::m31::PackedBaseField;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use crate::vm::{Op, VM};

impl FrameworkEval for VM {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut a = eval.next_trace_mask();
        let mut b = eval.next_trace_mask();
        self.program.iter().for_each(|op| {
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
