# NOIST: Non-interactive Single-round Threshold Signatures
[`NOIST`](https://blog.brollup.org/introducing-noist-a-non-interactive-single-round-t-of-n-threshold-signing-protocol-51225fe513fa) is a non-interactive, single-round t-of-n threshold signing scheme allowing multiple untrusted entities to jointly produce digital signatures in constant time, where a disruptive signer cannot force a re-do of the entire round. The resulting signature is a single 64-byte BIP-340 compatible Schnorr signature.

> [!WARNING]
> `NOIST` currently does not have a formal security proof.