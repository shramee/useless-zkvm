pub mod vm;
use useless_zkvm::utils::dummy_program;

fn main() {
    let result = dummy_program().run();
    println!("Result: {}", result);
}
