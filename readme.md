# Useless ZKVM on Stwo

This is a reconstruction of the fun little Plonky3 based ZKVM implementation by @armanthepythonguy called [Useless-ZKVM](https://github.com/armanthepythonguy/Useless-ZKVM).
This ZKVM is built on [Stwo](https://github.com/starkware-libs/stwo), prover holding the world record of most Poseidon hashes proved in a sec (Spoiler, a jawdropping 500k).

## Outline

Uses immutable use once memory maintained as trace for proving and a vec (array/list) of instructions to run on the provided inputs.

### Supported instructions:

* `Push(u32)`: Push a value onto the stack.
* `Add`: Add the top two stack values.
* `Sub`: Subtract the second stack value from the top.
* `Mul`: Multiply the top two stack values.
* `Div`: Divide the second stack value by the top.

## Defining programs

Programs are a sequence of instructions,
```rs 
use Op::{Add, Div, Mul, Push, Sub};
let program = Vec::from([
    Push(3),
    Push(7),
    Add,
    Push(2),
    Mul,
    Push(67),
    Div,
    Push(567),
    Sub,
]);

```
