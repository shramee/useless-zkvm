use itertools::Itertools;

use num_traits::Zero;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkEval};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedBaseField;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::{BaseField, M31};
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
            println!("Constraint {:?}: {:?} | {:?} | {:?}", op, a, b, c);
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
    let mut mem: Vec<M31> = Vec::new();
    vm.program().into_iter().for_each(|op| {
        // find value from the operation
        if let Op::Push(val) = op {
            mem.push(*val);
            println!("Trace: Psh {}", mem[mem.len() - 1]);
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
            println!("Trace: {:?} {}", op, mem[mem.len() - 1]);
        };
    });

    let mut trace: Vec<BaseColumn> = Vec::new();
    // Slightly ugly, but we don't wanna init with anything (not even zeroes)
    let mut mem_col = BaseColumn {
        data: vec![],
        length: 0,
    };

    while mem.len() > 0 {
        mem_col
            .data
            .push(PackedBaseField::from_array(std::array::from_fn(
                |_| match mem.pop() {
                    Some(val) => val,
                    None => BaseField::zero(),
                },
            )));
    }
    mem_col.length = mem_col.data.len();
    trace.push(mem_col);

    let domain = CanonicCoset::new(vm.log_size()).circle_domain();
    trace
        .into_iter()
        .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
        .collect_vec()
}
