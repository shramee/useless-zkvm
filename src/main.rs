use Op::{Add, Div, Mul, Push, Sub};
use Useless_ZKVM::vm::run_vm;

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
