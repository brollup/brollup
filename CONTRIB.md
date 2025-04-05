# Contributing
We welcome contributions! If you encounter any issues or have suggestions, please feel free to submit a pull request or open an issue.

## Areas for Contribution

### Optimized Rank-Based Indexing ⭐⭐⭐⭐⭐
Brollup uses a cached, rank-based indexing to efficiently encode and decode `Accounts` and `Contracts` via their compact integer reference values. For more context, see [CPE](https://github.com/brollup/brollup/tree/main/src/constructive/cpe).

We are looking for a performance-optimized implementation for the [Account Registry](https://github.com/brollup/brollup/blob/main/src/inscriptive/registery/account_registery.rs) and [Contract Registry](https://github.com/brollup/brollup/blob/main/src/inscriptive/registery/contract_registery.rs) to handle this rank system, at the memory level as performance optimized as possible.

### StackUint Refactor ⭐⭐
[StackUint](https://github.com/brollup/brollup/blob/main/src/executive/stack/stack_item/uint_ext.rs) is an unsigned stack integer representation with a variable byte length, supporting up to 256 bit unsigned integer values. Currently, it relies on the `uint` crate, and the `from_uint` function performs extra steps to truncate zero bytes to fit the corresponding byte length for the `StackItem`.

We are looking for a dependency-free, performance-optimized implementation, along with the new updated [tests](https://github.com/brollup/brollup/blob/main/tests/stack_uint.rs) coverage.

### Opcodes ⭐⭐
We strive to keep the platform as expressive as possible. As part of our open-source initiative, we encourage anyone to propose or implement new opcodes that are not yet present in the [wishlist](https://github.com/brollup/brollup/tree/main/src/executive/opcode), or suggest modifications to the existing list.

Additionally, Brollup replaces gas with ops, and further [contributions](Feel free to contribute to ops) are needed to evaluate the cost of executing opcodes.
