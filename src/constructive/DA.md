# Compact Payload Encoding
`Compact Payload Encoding (CPE)` is a custom-tailored algorithm designed to optimize data availability (DA) efficiency for Bitcoin.

`CPE` allows Brollup to pack as many transactions as possible into a Bitcoin block, enabling it to handle significantly more transactions compared to zkEVM and EVM. By optimizing transaction encoding, indexing, and signature aggregation, Brollup achieves higher throughput and lower transaction costs, making it a highly scalable solution for decentralized Bitcoin applications.

### VM Comparsion

| VM Type | Encoding                        | Scope      | Indexing       | Signature   | Gas Price/Limit | Error-handling | Efficiency |
|:--------|:--------------------------------|:-----------|:---------------|:------------|:----------------|:---------------|:-----------|
| Brollup | Compact-payload-encoding (CPE)  | Bit-level  | Rank-based     | Aggregated  | -               | Assertions     | 10.4x      |
| zkEVM   | Recursive-length prefix (RLP)   | Byte-level | Registery-based| Aggregated  | Present         | Failures       | 3.8x       |
| EVM     | Recursive-length prefix (RLP)   | Byte-level | -              | 65 bytes    | Present         | Failures       | 1x         |

`Compact Payload Encoding (CPE)`'s efficiency is attributed to 7 key areas:

#### 1. Bit-level CPE Encoding
Brollup uses bit-level encoding for transactions, as opposed to the standard byte-level encoding used by zkEVM and EVM. zkEVMs and EVMs use `Recursive-Length Prefix (RLP) encoding`, which breaks data into chunks of 8 bits (1 byte), leading to higher overhead. For instance, `u32` and `u64` values in RLP encoding take up 4 and 8 bytes, respectively. In contrast, Brollup uses a `compact-payload encoding (CPE)` at the bit level, where values are encoded in smaller, more efficient units, allowing more data to fit into the same space. 

Bit-level encoding isn't practical for zkEVMs due to the added complexity it introduces in generating ZKPs (zero-knowledge proofs). Brollup, on the other hand,  thanks to its use of bit-level encoding, achieves savings of 1-3 bytes for `u32` and 1-7 bytes for `u64`, with an added overhad of only 2-3 bits.

See [Valtype](https://github.com/brollup/brollup/tree/main/src/constructive/valtype).

#### 2. Rank-based Indexing
Brollup indexes accounts and contracts based on how frequently they transact rather than when they are registered. Every time an account transacts or is called, the rank of that account or contract is incremented. This rank-based indexing, handled at the memory level, ensures that frequently-used contracts (e.g., AMM pools, Tether) consume only 1 byte, compared to zkEVM’s 4 bytes and EVM’s 20 bytes.

#### 3. Common Value Lookup
Brollup uses a lookup table to efficiently encode commonly used values like 100, 5,000, and 10,000,000. This method significantly reduces byte usage when contracts with fewer decimal places are called with these values. By leveraging the lookup table to encode frequent patterns, Brollup minimizes DA overhead at scale. 

See [CommonVal](https://github.com/brollup/brollup/blob/main/src/constructive/valtype/maybe_common/common_val.rs).

#### 4. Signature Aggregation
Brollup aggregates transaction signatures using `MuSig2`, resulting in a constant 64-byte aggregated signature, instead of using ZKPs, which typically take up around 500 bytes. This results in a saving of 436 bytes per block compared to zkEVMs. 

See [Musig-nested-NOIST](https://blog.brollup.org/covenant-emulation-with-musig-nested-noist-784d428c7446).

#### 5. Non-prefixed Calldata Encoding
While Ethereum requires each calldata element to be prefixed with an `RLP` encoding (adding 1-2 bytes per element), Brollup directly maps calldata elements to pre-defined types with known lengths, eliminating the prefix overhead. 

See [Calldata](https://github.com/brollup/brollup/tree/main/src/constructive/calldata).

#### 6. Removed Fields: Gas Price and Gas Limit
Brollup replaces "gas" with "ops" (operations), simplifying transaction processing. The gas price and gas limit fields are removed because the transaction outcome is determined upon committing to a covenant session, where execution is inherently deterministic. Additionally, fee limits are enforced at the session level, not the block production level, further removing the need for these fields.

See [Entry](https://github.com/brollup/brollup/tree/main/src/constructive/entry).

#### 7. Assertions vs. Failures
In Brollup, transactions are asserted, meaning that only valid transactions are included in blocks. Failed transactions are never recorded, resulting in a cleaner state and fewer invalid operations. In contrast, both zkEVM and Ethereum allow failed transactions to end up in blocks, which increases overhead and reduces overall efficiency. This means Brollup achieves an overall 5% historical block space savings in comparison.

### Savings
Taking an average AMM contract call as an example, Brollup beats zkEVMs by 2.7x in terms of DA efficiency. This means Brollup can fit in 2.7x more transactions than a zkEVM clone on Bitcoin. Compared to a regular EVM, Brollup is nearly 10.4x more efficient, meaning it can fit in 10.4x more transactions.

| VM Type | From Account | To Contract | Method Call | ~Gas Price/Limit | ~Calldata    | Signature   | ~Size       | ~Savings    |
|:--------|:-------------|:------------|:------------|:-----------------|:-------------|:------------|:------------|:------------|
| Brollup | ~3 bytes     | 1 byte      | 3 bits      | -                | 6 bytes      | Negligible  | 10 bytes    | 94 bytes    |
| zkEVM   | 4 bytes      | 4 bytes     | 1 byte      | 8 bytes          | 10 bytes     | Negligible  | 27 bytes    | 77 bytes    |
| EVM     | -            | 20 bytes    | 1 byte      | 8 bytes          | 10 bytes     | 65 bytes    | 104 bytes   | -           |

> [!NOTE]
> This comparison excludes further savings from (3) common value lookup, (4) signature aggregation, and (7) assertions. Factoring in these optimizations, the efficiency is projected to surpass 3.0x compared to zkEVMs.