
# Non-interactive Single-session Threshold Signatures
NOIST is a non-interactive, single-session t-of-n threshold signature scheme allowing multiple untrusted entities to jointly produce digital signatures in constant time, where a disruptive signer cannot force a re-do of the entire signing session. The resulting signature is a single 64-byte [BIP-340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki) compatible Schnorr signature.

> [!WARNING]
> NOIST currently does not have a formal security proof.

## Key Features

### Abortion-proof Sessions
Signing sessions do not abort if a signatory produces an invalid partial signature or fails to fulfill the promise of producing a partial signature. Each signing session is guaranteed to yield a valid aggregate signature as long as the threshold is met.
 
### Non-interactive Signing
Partial signatures can be gathered without a time constraint (i.e., session timeout), as long as enough DKG packages are available for the group nonce. Otherwise, preprocessing must be run to populate the nonce pool with new DKG packages, which is an interactive process in itself.

## Algorithms
### Computing Encryption Keys
To compute the encryption keys, the secret key of the well-known signatory and the public key of the corresponding signatory are input to the _EncryptingKeySecret_ algorithm. The algorithm returns an encryption secret and public key pair, which will be used to encrypt FROST shares during the preprocessing phase.

Algorithm _EncryptionKeys(sk, PK)_:
-   Inputs:
    -   Secret key of the well-known signatory  _sk_: a secp scalar.
    -   Public key of the correspondent signatory  _PK_: a secp point.
-   Let  _E = sk • PK_.
-   Let  _h = H(cbytes(E))_.
-   Let _d' = int(h) mod n_.
-   Fail if  _d' = 0_.
-   Let P' = _d' • G_.
-   Let  _d  = d_ if  _has_even_y(P')_, otherwise let  _d  = n - d'_.
-   Let P = _lift_x(P')_.
-   Return  _d, P_.

### Encrypting Secret Shares
To encrypt a FROST share, the secret share produced by _SecretShareShard_ and the encryption secret key derived from _EncryptionKeys_ are input to the _ShareEncrypt_ algorithm. The algorithm returns the encrypted secret share, which can be safely transmitted over an insecure channel.

Algorithm _ShareEncrypt(sh, eks)_:
-   Inputs:
    -   Secret share  _ss_: a secp scalar.
    -   Encryption secret  _es_: a secp scalar.
-   Let  _ess = (ss + es) mod n_.
-   Fail if  _ess = 0_.
-   Return _ess_.

### Decrypting Secret Shares
To decrypt a FROST share, the encrypted secret share provided by a signatory and the encryption secret key derived from _EncryptionKeys_ are input to the _ShareDecrypt_ algorithm. The algorithm returns the original secret share, which is required for producing partial signatures.

Algorithm _ShareDecrypt(en, ess)_:
-   Inputs:
    -   Encrypted secret share  _ess_: a secp scalar.
    -   Encryption secret  _es_: a secp scalar.
-   Let  _ss = ess - es_.
-   Fail if  _ss = 0_.
-   Return _ss_.

### Verifying Shares
To verify the authenticity of a FROST share, the encrypted secret and public share pairs provided by a signatory, along with the encryption public key derived from _EncryptionKeys_, are input to the _ShareEncVerify_ algorithm. The algorithm returns whether the encrypted secret share has been encrypted correctly.

Algorithm _ShareEncVerify(ess, PS, EP)_:
-   Inputs:
    -   Encrypted secret share  _ess_: a secp scalar.
    -   Public share  _PS_: a secp point.
    -   Encryption public  _EP_: a secp point.
-   Let  _Q = ess • G_.
-   Return  _Q == PS + EP_.

## Pre-setup
Before forming a quorum, the intended signatories agree on the well-known public keys of all signatories. These well-known public keys are not linked to FROST shares and are used solely to identify signatories in subsequent sessions.

## Setup
NOIST encrypts FROST shares using encryption keys derived from a one-time setup via the Elliptic Curve Diffie-Hellman (ECDH) key exchange. These encryption keys are computed once and reused in subsequent sessions throughout the quorum's lifetime. FROST shares are encrypted in a manner that allows all parties to authenticate them, while ensuring they can only be decrypted by the intended signatory.

## Preprocessing

## Signing

## Partial Signature Verification

## Signature Aggregation

## Signature Verification

## Mutation