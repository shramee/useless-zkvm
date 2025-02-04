use itertools::Itertools;

use num_traits::Zero;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, TraceLocationAllocator,
};
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation, PolyOps};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{prove, verify, ProvingError, StarkProof};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::ColumnVec;

use crate::utils::one_row_col;
use crate::vm::{Op, VM};

pub type VMComponent = FrameworkComponent<VM>;

impl FrameworkEval for VM {
    fn log_size(&self) -> u32 {
        2
        // self.log_n_rows()
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
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
        trace.push(one_row_col(val, vm.log_n_rows() as usize));
    });

    let domain = CanonicCoset::new(vm.log_size()).circle_domain();
    trace
        .into_iter()
        .map(|col| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, col))
        .collect_vec()
}

pub fn prove_vm(
    vm: VM,
) -> (
    Result<StarkProof<Blake2sMerkleHasher>, ProvingError>,
    VMComponent,
) {
    let config = PcsConfig::default();
    // Precompute twiddles.
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(vm.log_n_rows() + 1 + config.fri_config.log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let prover_channel = &mut Blake2sChannel::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(config, &twiddles);

    // Preprocessed trace
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals([]);
    tree_builder.commit(prover_channel);

    // Trace.
    let trace = generate_vm_trace(&vm);
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(trace);
    tree_builder.commit(prover_channel);

    // Prove constraints.
    let component = VMComponent::new(
        &mut TraceLocationAllocator::default(),
        vm,
        SecureField::zero(),
    );

    (
        prove::<SimdBackend, Blake2sMerkleChannel>(
            &[&component],
            prover_channel,
            commitment_scheme,
        ),
        component,
    )
}

pub fn verify_vm(proof: StarkProof<Blake2sMerkleHasher>, component: VMComponent) {
    let config = PcsConfig::default();
    let verifier_channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

    // Retrieve the expected column sizes in each commitment interaction, from the AIR.
    let sizes = component.trace_log_degree_bounds();
    commitment_scheme.commit(proof.commitments[0], &sizes[0], verifier_channel);
    commitment_scheme.commit(proof.commitments[1], &sizes[1], verifier_channel);
    verify(&[&component], verifier_channel, commitment_scheme, proof).unwrap();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num_traits::Zero;
    use stwo_prover::{
        constraint_framework::{assert_constraints, FrameworkEval, TraceLocationAllocator},
        core::{
            air::Component,
            backend::simd::SimdBackend,
            channel::Blake2sChannel,
            fields::qm31::SecureField,
            pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig, TreeVec},
            poly::circle::{CanonicCoset, PolyOps},
            prover::{prove, verify},
            vcs::blake2_merkle::Blake2sMerkleChannel,
        },
    };

    use crate::{
        prover::{generate_vm_trace, VMComponent},
        utils::dummy_program,
    };

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

    #[test_log::test]
    fn test_vm_prove_with_blake() {
        let vm = dummy_program();
        let config = PcsConfig::default();
        // Precompute twiddles.
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(vm.log_n_rows() + 1 + config.fri_config.log_blowup_factor)
                .circle_domain()
                .half_coset,
        );

        // Setup protocol.
        let prover_channel = &mut Blake2sChannel::default();
        let mut commitment_scheme =
            CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(config, &twiddles);

        // Preprocessed trace
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals([]);
        tree_builder.commit(prover_channel);

        // Trace.
        let trace = generate_vm_trace(&vm);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(prover_channel);

        // Prove constraints.
        let component = VMComponent::new(
            &mut TraceLocationAllocator::default(),
            vm,
            SecureField::zero(),
        );

        let proof = prove::<SimdBackend, Blake2sMerkleChannel>(
            &[&component],
            prover_channel,
            commitment_scheme,
        )
        .unwrap();

        // Verify.
        let verifier_channel = &mut Blake2sChannel::default();
        let commitment_scheme = &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

        // Retrieve the expected column sizes in each commitment interaction, from the AIR.
        let sizes = component.trace_log_degree_bounds();
        commitment_scheme.commit(proof.commitments[0], &sizes[0], verifier_channel);
        commitment_scheme.commit(proof.commitments[1], &sizes[1], verifier_channel);
        verify(&[&component], verifier_channel, commitment_scheme, proof).unwrap();
    }
}
