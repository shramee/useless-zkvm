use itertools::Itertools;

use stwo_prover::constraint_framework::{EvalAtRow, FrameworkEval};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use crate::utils::one_row_col;
use crate::vm::{Op, VM};

impl FrameworkEval for VM {
    fn log_size(&self) -> u32 {
        1
        // self.log_n_rows()
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        1
        // self.log_n_rows() + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut a = eval.next_trace_mask();
        let mut b = eval.next_trace_mask();
        self.program().iter().enumerate().for_each(|(i, op)| {
            if i < 2 {
                if let Op::Push(_) = op {
                    // all good
                } else {
                    panic!("first two operations should be Push");
                }
                return;
            }
            let c = eval.next_trace_mask();
            if let Op::Push(_) = op {
                // nothing here
            } else {
                if let Some(constraint) = match op {
                    Op::Add => Some(c.clone() - (a.clone() + b.clone())), // c=a+b
                    Op::Sub => Some(c.clone() - (a.clone() - b.clone())), // c=a-b
                    Op::Mul => Some(c.clone() - (a.clone() * b.clone())), // c=a*b
                    Op::Div => Some(a.clone() - (c.clone() * b.clone())), // a=c*b
                    _ => None,
                } {
                    eval.add_constraint(constraint);
                    println!("Constraint {:?}: {:?} | {:?} | {:?}", op, a, b, c);
                }
            }

            // swapping to avoid cloning, a = b, b = c, c is the next element on the trace
            std::mem::swap(&mut a, &mut b);
            b = c;
        });
        eval
    }
}

pub fn generate_vm_trace(
    vm: &VM,
) -> ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
    let mut mem: Vec<M31> = Vec::new();
    vm.program().into_iter().for_each(|op| {
        // find value from the operation
        if let Op::Push(val) = op {
            mem.push(*val);
        } else {
            let b = mem[mem.len() - 1];
            let a = mem[mem.len() - 2];
            match op {
                Op::Add => mem.push(a + b),
                Op::Sub => mem.push(a - b),
                Op::Mul => mem.push(a * b),
                Op::Div => mem.push(a / b),
                _ => {}
            };
            println!("Trace: {:?}: {:?} | {:?}", op, a, b);
        };
    });

    let mut trace: Vec<BaseColumn> = Vec::new();
    mem.into_iter().for_each(|val| {
        trace.push(one_row_col(val, 2));
    });

    let domain = CanonicCoset::new(vm.log_size()).circle_domain();
    trace
        .into_iter()
        .map(|col| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, col))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num_traits::Zero;
    use stwo_prover::{
        constraint_framework::{assert_constraints, FrameworkEval},
        core::{fields::qm31::SecureField, pcs::TreeVec, poly::circle::CanonicCoset},
    };

    use crate::{prover::generate_vm_trace, utils::dummy_program};

    #[test]
    fn test_vm_constraints() {
        let vm = dummy_program();

        let traces = TreeVec::new(vec![vec![], generate_vm_trace(&vm)]);
        let trace_polys =
            traces.map(|trace| trace.into_iter().map(|c| c.interpolate()).collect_vec());

        let num_rows = 1;

        assert_constraints(
            &trace_polys,
            CanonicCoset::new(vm.log_size()),
            |e| {
                if e.row < num_rows {
                    vm.evaluate(e);
                }
            },
            SecureField::zero(),
        );
    }
}
