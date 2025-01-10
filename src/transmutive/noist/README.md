= Non-interactive Single-round Threshold Signatures =

== Algorithms ==
=== Computing the encryption keys ===
To compute the encryption secret key, the secret key of the well-known signatory and the public key of the corresponding signatory are provided to ''EncryptingKeySecret''. The algorithm returns the encryption secret and public key pair, which will be used to encrypt FROST shares in subsequent sessions.

'''Algorithm ''EncryptionKeys(sk, PK)''':'''
* Inputs:
** Secret key of the well-known signatory ''sk'': a secp scalar.
** Public key of the correspondent signatory ''PK'': a secp point.
* Let ''E = sk⋅pk''.
* Let ''h = H(cbytes(E))''.
* Let ''d' = int(h) mod n''.
* Fail if ''d' = 0''.
* Let ''P' = d'⋅G''.
* Let ''d = d'' if ''has_even_y(P')'', otherwise let ''d = n - d'''.
* Let ''P = lift_x(P')''.
* Return ''d, P''.

=== Computing the encryption public ===
To compute the encryption secret key:

'''Algorithm ''EncryptionPublic(sk, pk)''':'''
* Inputs:
** Self secret key ''sk'': a secp scalar.
** Correspondent public key ''pk'': a secp point.
* Let ''d = EncryptingKeySecret(sk, pk)''.
* Let ''P = d * G''.
* Return ''P''.

=== Encrypting secret share ===
'''Algorithm ''ShareEncrypt(sh, eks)''':'''
* Inputs:
** Secret share ''sh'': a secp scalar.
** Encrypting key secret ''eks'': a secp scalar.
* Let ''enc = sh + eks''.
* Fail if ''enc = 0'' or ''enc ≥ n''.
* Return ''enc''.

=== Decrypting secret share ===
'''Algorithm ''ShareDecrypt(enc, eks)''':'''
* Inputs:
** Encrypted secret share ''enc'': a secp scalar.
** The encrypting key secret ''eks'': a secp scalar.
* Let ''sh = enc - eks''.
* Fail if ''sh = 0'' or ''sh ≥ n''.
* Return ''sh''.

== Setup ==
NOIST encrypts FROST shares using encryption keys derived from a one-time setup via the Elliptic Curve Diffie-Hellman (ECDH) key exchange. These encryption keys are computed once and reused in subsequent sessions throughout the quorum's lifetime. FROST shares are encrypted in a manner that allows all parties to authenticate them, while ensuring they can only be decrypted by the intended signatory.

== Nonce Preprocessing ==

== Signing ==

== Partial Signature Verification ==

== Signature Aggregation ==
