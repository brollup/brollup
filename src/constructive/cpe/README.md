# Compact Payload Encoding
`Compact Payload Encoding (CPE)` is a custom-tailored algorithm designed to optimize data availability (DA) efficiency for Bitcoin.

`CPE` allows Brollup to pack as many transactions as possible into a Bitcoin block, enabling it to handle significantly more transactions compared to zkEVM and EVM. By optimizing transaction encoding, indexing, and signature aggregation, Brollup achieves higher throughput and lower transaction costs, making it a highly scalable solution for decentralized Bitcoin applications.

### VM Comparsion
| VM Type | Encoding                        | Scope      | Indexing       | Signature   | Nonce     | Gas Price/Limit | Error-handling | Efficiency |
|:--------|:--------------------------------|:-----------|:---------------|:------------|:----------|:----------------|:---------------|:-----------|
| Brollup | Compact-payload-encoding (CPE)  | Bit-level  | Rank-based     | Aggregated  | -         | -               | Assertions     | 10.3x      |
| zkEVM   | Recursive-length prefix (RLP)   | Byte-level | Registery-based| Aggregated  | Present   | Present         | Failures       | 3.3x       |
| EVM     | Recursive-length prefix (RLP)   | Byte-level | -              | 65 bytes    | Present   | Present         | Failures       | 1x         |

`Compact Payload Encoding (CPE)`'s efficiency is attributed to 8 key areas:

#### 1. Bit-level Encoding
`CPE` uses bit-level encoding for transaction and value types, unlike the standard byte-level `RLP` encoding used by zkEVM and EVM. 

While `RLP` encoding requires 4 bytes for `u32` and 8 bytes for `u64`, `CPE` encodes these values in smaller units, allowing more data to fit into the same space. This results in savings of 1-3 bytes for `u32` and 1-7 bytes for `u64`, with only a 2-3 bit overhead.

Bit-level encoding is impractical for zkEVMs due to the increased complexity in generating zero-knowledge proofs (ZKPs), as it demands more precise data handling.

See [Valtype](https://github.com/brollup/brollup/tree/main/src/constructive/valtype).

#### 2. Rank-based Indexing
Brollup indexes `Accounts` and `Contracts` based on how frequently they transact, rather than when they are registered. Each time an `Account` initiates a transaction or a `Contract` is called, their rank is incremented by one.

This rank-based indexing system is cached and managed at the memory level, ensuring that frequently used contracts—such as AMM pools or Tether—consume only ~1 byte, compared to zkEVM’s 4 bytes and EVM’s 20 bytes.

#### 3. Non-prefixed Calldata
`CPE` maps calldata items directly to pre-defined types with known lengths, eliminating the prefix overhead for calldata. In contrast, the EVM requires calldata to be prefixed with an `RLP` encoding, adding 1-2 bytes overhead.

See [Calldata](https://github.com/brollup/brollup/tree/main/src/constructive/calldata).

#### 4. Compact Call Method
`CPE` decodes `Contract` call methods through a varying bitsize `AtomicVal`.

In the case of an average `Contract` with four callable methods, `AtomicVal` would consume only 2 bits. In contrast, traditional EVM function selectors require 4 bytes. This results in a savings of 30 bits per `Entry` in the `Payload`, translating to an approximate ~0.93 vBytes of block space savings.

#### 5. Common Value Lookup
`CPE` uses a lookup table to efficiently encode commonly used values like 100, 5,000, and 10,000,000. This method significantly reduces byte usage when contracts with fewer decimal places are called with these values. By leveraging the lookup table to encode frequent patterns, Brollup minimizes DA overhead at scale. 

See [CommonVal](https://github.com/brollup/brollup/blob/main/src/constructive/valtype/maybe_common/common_val.rs).

#### 6. Signature Aggregation
Brollup aggregates transaction signatures using `MuSig2`, resulting in a constant 64-byte aggregated signature, instead of using ZKPs, which typically take up around 500 bytes. This results in a saving of 436 bytes per block compared to zkEVMs. 

See [Musig-nested-NOIST](https://blog.brollup.org/covenant-emulation-with-musig-nested-noist-784d428c7446).

#### 7. Field Cleanup
`CPE` omits the nonce, gas price, and gas limit from the transaction encoding scheme, while they are still commited to the `Entry` sighash.

Brollup replaces `Gas` with `Ops` and enforces the `Ops limit` at the session level, rather than the transaction level. Similarly, `Ops price` is encoded at the `Payload` level and applied to all entries within the `Payload`.

Additionally, Brollup removes the need for a nonce field to track internal transaction states. Since rollup state transitions are externally chained, the requirement for internal chaining is eliminated.

See [Entry](https://github.com/brollup/brollup/tree/main/src/constructive/entry).

#### 8. Assertions
In Brollup, transactions are asserted, meaning that only valid transactions are included in blocks. Failed transactions are never recorded, resulting in a cleaner state and fewer invalid operations. In contrast, both zkEVM and Ethereum allow failed transactions to end up in blocks, which increases overhead and reduces overall efficiency. This means Brollup achieves an overall 5% historical block space savings in comparison.

### Savings
Taking an average AMM contract call as an example, Brollup consumes only around ~10.6 bytes (~2.65 vBytes) of block space. In comparison, zkEVM requires 33 bytes, resulting in an approximate savings of 5.6 vBytes of block space. This makes Brollup roughly 3.30x times more efficient in terms of data availability. As a result, Brollup can handle about 3.10x times more transactions than a zkEVM clone on Bitcoin and approximately x10.3 times more transactions than a standard EVM.

| VM Type | From Account | To Contract | Call Method | Nonce    | Gas Price/Limit | Call Method | Calldata   | Signature  | Size       | Savings    |
|:--------|:-------------|:------------|:------------|:---------|:----------------|:------------|:-----------|:-----------|:-----------|:-----------|
| Brollup | ~3 bytes     | ~10 bits    | 2 bits      | -        | -               | 2 bits      | 6 bytes    | Negligible | 10.6 bytes | 99.4 bytes |
| zkEVM   | 4 bytes      | 4 bytes     | 1 byte      | ~3 bytes | ~8 bytes        | 4 bytes     | 9 bytes    | Negligible | 33 bytes   | 77 bytes   |
| EVM     | -            | 20 bytes    | 1 byte      | ~3 bytes | ~8 bytes        | 4 bytes     | 9 bytes    | 65 bytes   | 110 bytes  | -          |

> [!NOTE]
> This comparison excludes further savings from (3) common value lookup, (4) signature aggregation, and (7) assertions. Factoring in these optimizations, the efficiency is projected to surpass 3.10x compared to zkEVMs.