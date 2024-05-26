# Unused memory load on LLVM generated code

## Summary

Some compiler frontends such as `Rust` or `Julia` can create assembly code with memory load that may not be used in under some conditions.

## Before you read

### Memory load

Assembly code has several different instructions for memory load.  
`MOV` is the only memory load instruction that appears on this article.

### Phi node

Phi node is a battle tested optimization technique that has been around for decades.  
This artcile believes that phi node optimization is the root cause of the unused memory load.

## How To Reproduce Unused Memory Load  

Here I provide you with a minimal reproducible example.  

In both examples, you can find see that there is a `MOV` instruction before the start of the loop, which will not be used unless the subsequent `JE` instruction evaluates to equal.

```text
SIDE NOTE:

- MOV  
MOV instruction moves a piece of to register.  
This instruction is slow when the data is on RAM. If the data is accessed on CPU cache or another register.

- JE
JE instruction is an instruction that jumps only if the boolean flag is true.
```

### Example 1: Rust

[Compiler Explorer Link](https://rust.godbolt.org/z/xTo3dfG7E)

```rust

```

```asm

```

### Julia  

Compiler Explorer Link:  

```rust

```

```asm

```

## Why is it happening?

LLVM has number of optimization passes to eliminate unecessary memory operations, it is clear that this one is left uncaught.

CFG of generated IR shows that it doesn't have any redundant blocks; However, one of the block caught on `phi` node has memory load instruction whose loaded data will not be used if the loop doesn't happen.
