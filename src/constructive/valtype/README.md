# Valtype

`Brollup` employs 3 specialized value types to minimize Bitcoin block space consumption, thereby optimizing data availability (DA) efficiency. The protocol is designed to ensure that transaction summary data is encoded in a highly compact form before being published in the `Payload` field of on-chain rollup state transitions.

The 3 value types in `Brollup` are defined as follows:

| Value Type | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| AtomicVal  | A highly compact integer representation ranging from 0 to 7. Primarily used for encoding minimal values such as contract method call indexes. |
| ShortVal   | A flexible integer representation spanning from 0 to 4,294,967,295. Designed for storing small-scale values, such as contract or account registry indexes. |
| LongVal    | A scalable integer representation accommodating values from 0 to 18,446,744,073,709,551,615. Intended for large numerical data, such as contract parameters. |

## AtomicVal

`AtomicVal` is a compact unsigned integer representation ranging from 0 (inclusive) to 7 (inclusive). It is used for representing very small values, such as contract method call indexes. `AtomicVal` consumes only 3 bits, compared to the `ShortVal` equivalent (`u8`), which would require 8 + 2 = 10 bits. This results in a savings of 7 bits per `Entry` in the `Payload`, translating to approximately ~0.22 vBytes of block space savings.

## ShortVal

`ShortVal` is a compact unsigned integer representation ranging from 0 (inclusive) to 4,294,967,295 (inclusive). It is used for representing small values such as contract or account registry indexes. `ShortVal` consumes a varying amount of space, calculated as `2 + 8n` bits, where `n` corresponds to the tier of `ShortVal`. There are four tiers:

| ShortVal Tiers | Description                                | Bitsize |
|----------------|--------------------------------------------|---------|
| U8             | Represents values from 0 to 255.           | 10 bits |
| U16            | Represents values from 0 to 65,535.        | 18 bits |
| U24            | Represents values from 0 to 16,777,215.    | 26 bits |
| U32            | Represents values from 0 to 4,294,967,295. | 34 bits |

## LongVal

`LongVal` is a compact unsigned integer representation ranging from 0 (inclusive) to 18,446,744,073,709,551,615 (inclusive). It is used for representing large values such as contract parameters. `LongVal` consumes a varying amount of space, calculated as `3 + 8n` bits, where `n` corresponds to the tier of `LongVal`. There are eight tiers:

| LongVal Tiers| Description                                            | Bitsize|
|--------------|--------------------------------------------------------|---------|
| U8           | Represents values from 0 to 255.                       | 11 bits |
| U16          | Represents values from 0 to 65,535.                    | 19 bits |
| U24          | Represents values from 0 to 16,777,215.                | 27 bits |
| U32          | Represents values from 0 to 4,294,967,295.             | 35 bits |
| U40          | Represents values from 0 to 1,099,511,627,775.         | 43 bits |
| U48          | Represents values from 0 to 281,474,976,710,655.       | 51 bits |
| U56          | Represents values from 0 to 72,057,594,037,927,935.    | 59 bits |
| U64          | Represents values from 0 to 18,446,744,073,709,551,615.| 67 bits |

This tiered structure ensures efficient use of space while encoding transaction data in `Brollup`. 

