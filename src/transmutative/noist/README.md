
# Non-interactive Single-session Threshold Signatures
NOIST is a non-interactive, single-session t-of-n threshold signature scheme allowing multiple untrusted entities to jointly produce digital signatures in constant time, where a disruptive signer cannot force a re-do of the entire signing session. The resulting signature is a single 64-byte [BIP-340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki) compatible Schnorr signature.

> [!WARNING]
> NOIST currently does not have a formal security proof.

## Key Features

### Abortion-proof
Signing sessions do not abort if a signatory produces an invalid partial signature or fails to fulfill the promise of producing a partial signature. Each signing session is guaranteed to yield a valid aggregate signature as long as the threshold is met.
 
### Non-interactive
Signing sessions can run without time constraints, enabling partial signatures to be collected from offline clients (e.g., hardware wallets) or semi-uptime clients (e.g., smartphones) without a session timeout.

### Nonce Pooling
The group nonce becomes available immediately upon entering a signing session, provided that enough DKG packages are available. Otherwise, preprocessing must be run to populate the nonce pool with new DKG packages, which is an interactive process.

## Core Algorithms

### Schnorr
#### Schnorr Sign
The algorithm _SchnorrSign_ returns a digital signature against a message and a public key.
Algorithm _SchnorrSign(sk, m)_:
-   Inputs:
    -   Secret key of the well-known signatory  _sk_: a secp scalar.
    -   Message to sign _m_: a secp scalar.
-   Let secret key _sk'  = sk_ if  _has_even_y(P')_, otherwise let  _sk'  = n - sk_.
-   Let public key P = sk' • G_.
-   Let secret nonce _k_ = a secp scalar freshly generated uniformly at random.
-   Let secret nonce _sn'  = sn_ if  _has_even_y(P')_, otherwise let  _sn'  = sn - k_.
-   Let public nonce _R = sn' • G_.
-   Let challenge _e = H(H("BIP0340/challenge") || H("BIP0340/challenge") + R || P || m)_.
-   Let signature _s = sn' + (e * sk')_.
-   Return _R, s_.

#### Schnorr Verify
The algorithm _SchnorrVerify_ checks if a digital signature is valid for a message and a public key.
Algorithm _SchnorrVerify(P, m, s, R)_:
-   Inputs:
    -   Public key of the well-known signatory  _P_: a secp point.
    -   Message to verify _m_: a secp scalar.
    -   Signature _s_: a secp scalar.
    -   Public nonce _R_: a secp point.
-   Let challenge _e = H(H("BIP0340/challenge") || H("BIP0340/challenge") + R || P || m)_.
-   Return _s • G == R + (e * P)_.

### Lagrance

#### Lagrance Index
The algorithm _LagranceIndex_ returns the index location of a signatory in a list of all signatories, determined by the lexicographical ordering of well-known public keys.

Algorithm _LagranceIndex(PK, PK[1..n])_:
-   Inputs:
    -   Public key of the well-known signatory  _PK_: a secp point.
    -   List of all well-known signatories  _PK[1..n]_: a list of secp points.
-   Let _PK'[1..n] = PK[1..n]_ sorted in lexicographical order.
-   Return the index of _PK_ in _PK'[1..n]_.

#### Lagrance Index List
The algorithm _LagranceIndexList_ provides the index locations of a subset of signatories within a list of all signatories, based on the lexicographical order of well-known public keys.

Algorithm _LagranceIndexList(T[1..t], N[1..n])_:
-   Inputs:
    -   List of threshold number of well-known signatories  _T[1..t]_: a list of secp points.
    -   List of all well-known signatories  _N[1..n]_: a list of secp points.
-   Let _T'[1..T] = T[1..T]_ sorted in lexicographical order.
-   Let _R[]_ be an empty list with length _t_.
-   For _i = 1 .. t_:
    -    Let _li = LagranceIndex(T[i], N[1..n])_.
    -    Insert _li_ into _R[]_.
- Return _R[1..t]_.

#### Lagrance Interpolating Value
The algorithm _InterpolatingValue_ returns the polynomial interpolating value for a signatory, given the signatory's Lagrange index and a list of Lagrange indexes meeting the threshold number.

Algorithm _InterpolatingValue(li, li[1..n])_:
-   Inputs:
    -   Lagrance index of a signatory  _li_: a secp scalar.
    -   List of threshold number of lagrance indexes  _li[1..t]_: a list of secp scalars.

-   Fail if _li_ not in _li[1..n]_.
-   Let num, den = 1, 1.
-   For _li_j in li[1..n]_:
    - If _li_j == li_ continue.
    - _num *= li_j_.
    - _den *= (li_j - li)_
-   Return _num / den_.

### Verifiable Secret Encryption

#### Computing Encryption Keys
To compute the encryption keys, the secret key of the well-known signatory and the public key of the corresponding signatory are input to the _EncryptingKeySecret_ algorithm. The algorithm returns an encryption secret and public key pair, which will be used to encrypt FROST shares during the preprocessing phase.

Algorithm _EncryptionKey(sk, PK)_:
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

#### Encrypting Secret Shares
To encrypt a FROST share, the secret share produced by _SecretShard_ and the encryption secret key derived from _EncryptionKey_ are input to the _ShareEncrypt_ algorithm. The algorithm returns the encrypted secret share, which can be safely transmitted over an insecure channel.

Algorithm _ShareEncrypt(ss, es)_:
-   Inputs:
    -   Secret share  _ss_: a secp scalar.
    -   Encryption secret  _es_: a secp scalar.
-   Let  _ess = (ss + es) mod n_.
-   Fail if  _ess = 0_.
-   Return _ess_.

#### Decrypting Secret Shares
To decrypt a FROST share, the encrypted secret share provided by a signatory and the encryption secret key derived from _EncryptionKey_ are input to the _ShareDecrypt_ algorithm. The algorithm returns the original secret share, which is required for producing partial signatures.

Algorithm _ShareDecrypt(ess, es)_:
-   Inputs:
    -   Encrypted secret share  _ess_: a secp scalar.
    -   Encryption secret  _es_: a secp scalar.
-   Let  _ss = ess - es_.
-   Fail if  _ss = 0_.
-   Return _ss_.

#### Verifying Shares
To verify the authenticity of a FROST share, the encrypted secret and public share pairs provided by a signatory, along with the encryption public key are input to the _ShareEncVerify_ algorithm. The algorithm returns whether the encrypted secret share has been encrypted correctly.

Algorithm _ShareEncVerify(ess, PS, EP)_:
-   Inputs:
    -   Encrypted secret share  _ess_: a secp scalar.
    -   Public share  _PS_: a secp point.
    -   Encryption public  _EP_: a secp point.
-   Let  _Q = ess • G_.
-   Return  _Q == PS + EP_.

### Verifiable Secret Sharing

#### Commiting To Shares
The algorithm _CommitShares_ takes a list of polynomial coefficients and returns the list of share commitments.

Algorithm _CommitShares(co[1..t])_:
-   Inputs:
    -   List of polynomial coefficients  _co[1..t]_: a list of secp scalars.
-   Let _COM[]_ be an empty list with length _t_.
-   For _i = 1 .. t_:
    -   _COM_i = co[i] • G_
    -   Insert _COM_i_ into _COM[]_.
-   Return _COM[1..t]_.

#### Verifying Share Commitments
The algorithm _VerifyShare_ takes the lagrance index and public share of a signatory, and a list of all share commitments.

Algorithm _VerifyShare(li, PS, COM[1..t])_:
-   Inputs:
    -   Lagrance index of the signatory _li_: a secp scalar.
    -   Public share of the signatory _PS_: a secp point.
    -   List of share commitments _COM[1..t]_: a list of secp points.
-   Let _P_ = a secp point at infinity.
-   For _i .. t_:
    -   _P += COM[i] • (li ^ i mod n)_
-   Return _P == PS_.

### Shares 

#### Polynomial Evaluation
The algorithm _PolynomialEval_ evaluates a polynomial based on a given set of coefficients and a specified x-position.

Algorithm _PolynomialEval(x_i, co[1..t])_:
-   Inputs:
    -   X position at which to evaluate the polynomial _x_i_: a secp scalar.
    -   A list of threshold number of coefficients _co[1..t]_: a list of secp scalars.
-   Let _val = 0_.
-   For _co_i in co[1..t]_:
    -  _val *= x_i_.
    -  _val += co_i_.
-   Return _val_.

#### Sharding Shares
The algorithm _ShareShard_ takes the polynomial secret, a list of additional secret coefficients, and the number of shares, then divides the polynomial secret into the specified number of shares.

Algorithm _ShareShard(s, co[1..t-1], n)_:
-   Inputs:
    -   The polynomial secret _s_: a secp scalar.
    -   A list of polynomial coefficients _co[1..t-1]_: a list of secp scalars.
    -   Number of total shares _n_: a secp scalar.
-   Let _co' = s + co[1..t-1]_.
-   Let _ss[]_ be an empty list with length _n_.
-   For _x_i = 1 .. n+1_:
    -   Let _y_i = PolynomialEval(x_i, co')_.
    -   Insert _y_i into ss[]_.
-   Return _ss[0..n], co'_.

#### Polynomial Generation
The algorithm _GenPolynomial_ generates a polynomial with coefficients drawn uniformly at random and returns a list of shares and their commitments generated by evaluating the polynomial.

Algorithm _GenPolynomial(s, t, n)_:
-   Inputs:
    -   The polynomial secret _s_: a secp scalar.
    -   Threshold value _t_: a secp scalar.
    -   Number of total shares _n_: a secp scalar.

-   Let _co[]_ be an empty list with length _t-1_.
-   Let _COM[]_ be an empty list with length _t-1_.
-   For _i = 0 .. t-1_:
    -   Let _co_i_ = a secp scalar freshly generated uniformly at random.
    -   Insert _co_i into co[]_.
-   Let _ss[], co' = ShareShard(s, co[1..t-1], n)_.
-   Let _COM[t] = CommitShares(co')_.
-   Return _ss[n], C[t]_.

## Pre-setup
Before forming a quorum, the intended signatories agree on the well-known public keys of all signatories. These well-known public keys are not linked to FROST shares and are used solely to identify signatories in subsequent sessions.

## Setup
NOIST encrypts FROST shares using encryption keys derived from a one-time setup via the Elliptic Curve Diffie-Hellman (ECDH) key exchange. These encryption keys are computed once and reused in subsequent sessions throughout the quorum's lifetime. FROST shares are encrypted in a manner that allows all parties to authenticate them, while ensuring they can only be decrypted by the intended signatory.

For signatories _i .. n_ the coordinator collects encryption public keys from all signatories:
-   For _i = 0 .. n_:
    -   Let be _EP_i[]_ an empty list with length n.
    -   For j = 0 .. n:
        -  _- , EP_i_j = EncryptionKey(sk_i, PK_j)_.
        -  Insert _EP_i_j_ into _EP_i[]_.
        -  Return _EP_i[]_.
    -   Return _i, EP_i[]_.

After collecting all contributions, the coordinator verifies that the encryption keys match.

-   For each _i, EP_i[1..n]_:
    -   For k = 0 .. n:
        -   Continue if _k == i_.
        -   Fail if _EP_i[k] != EP_k[i]_.

If the above fails, the coordinator must manually re-adjust the signatory set and re-run the setup. Currently, there is no identifiable aborts for identifying malicious setup contributions.

## Preprocessing
NOIST works by periodically running Distributed Key Generation (DKG) sessions to stockpile DKG packages. These DKG packages are subsequently used in signing sessions to construct the group nonce. This ensures that as long as a sufficient number of DKG packages are available, the group nonce becomes known at the start of a signing session.

#### Retrieving DKG Packages

Algorithm _RetrievePackages(PK[1..n])_:
-   Inputs
    -   Signatory list _PK[1..n]_: list of secp points.
-   Let _PK[1..n]_ be the signatory list, containing the well-known public keys of all signatories.
-   Let _t = (n / 2) + 1_ be the threshold.
-   Let _p_ represent the DKG package index, incremented by one for each package. When _p_ is zero, the package is used for constructing the group key. When _p_ is one or greater, the package is used for constructing group nonces.
-   Let _PKG_p[]_ be an empty list with the index _p_, holding DKG packages with a length of _m_, where _m ≥ t_.
-   For _i = 0 .. n_ retrieve packages from all signatories:
    -   Let _ess_h_i[], ess_b_i[]_ be two empty lists for hiding and binding encrypted secret shares, respectively, each with a length of _n_.
    -   Let _PS_h_i[], PS_b_i[]_ be two empty lists for hiding and binding public shares, respectively, each with a length of _n_.
    -   Let _s_h, s_b_ = two secp scalars freshly generated uniformly at random.
    -   Let _ss_h_i[1..n], COM_h_i[1..t] = GenPolynomial(s_h, t, n)_.
    -   Let _ss_b_i[1..n], COM_b_i[1..t] = GenPolynomial(s_b, t, n)_.
    -   For _z = 0 .. n_:
         -  Let _PS_h_i_z = ss_h_i[z] • G_.
         -  Insert _PS_h_i_z into PS_h_i[]_.
         -  Let _PS_b_i_z = ss_b_i[z] • G_.
         -  Insert _PS_b_i_z into PS_b_i[]_.
    -   For j = 0 .. n:
         -  _es_i_j = EncryptionKey(sk_i, PK_j)_.
         -  _ess_h_i_j = ShareEncrypt(ss_h_i[j], es_i_j)_.
         -  _ess_b_i_j = ShareEncrypt(ss_b_i[j], es_i_j)_.
         -  Insert _j, ess_h_i_j_ into _ess_h_i[]_.
         -  Insert _j, ess_b_i_j_ into _ess_b_i[]_.
    -   Let _pkg_i = PK_i, ess_h_i[1..n], PS_h_i[1..n], COM_h_i[1..t], ess_b_i[1..n], PS_b_i[1..n], COM_b_i[1..t]_.
    -   Let _m_i = H(bytes(p) || bytes(pkg_i))_.
    -   Let _sig_i = SchnorrSign(m_i, sk_i)_.
    -   Insert _sig_i_, _pkg_i_ into _PKG_p[]_.
-   Return _PKG_p[0..m]_ with length _m_ where _m >= t_.

#### Verifying DKG Packages

Algorithm _VerifyPackages(PK[1..n], PKG_p[1..m])_:
-   Inputs:
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with index _p_ and length _m_.

-   Let _t = (n / 2) + 1_ be the threshold.
-   Let _ph_ be the historical _p_ height.
-   Fail if _p <= ph_.
-   For _i = 0 .. m_:.
    -  Let _sig_i, pkg_i = PKG_p[i]_.
    -  Let PK_i, ess_h_i[1..n], PS_h_i[1..n], COM_h_i[1..t], ess_b_i[1..n], PS_b_i[1..n], COM_b_i[1..t] = pkg_i_.
    -  Let _m_i = H(bytes(p) || bytes(pkg_i))_.
    -  Fail if _!SchnorrVerify(m_i, PK_i, sig_i)_.
    -  For _k = 1 .. n_:
       -  Let li_k = _LagranceIndex(PK[k], PK[1..n])_.
       -  Let _PS_h_i_k, PS_b_i_k = PS_h_i[k], PS_b_i[k]_.
       -  Fail if _!VerifyShare(li_k, PS_h_k, COM_h_i[1..t]) || !VerifyShare(li_k, PS_b_k, COM_b_i[1..t])_.
       -  Let _ess_h_i_k, ess_b_i_k = ess_h_i[k], ess_b_i[k]_.
       -  Let _EP_i_k_ be the encryption public key from the setup phase.
       -  Fail if _!ShareEncVerify(ess_h_i_k, PS_h_i_k, EP_i_k) || !ShareEncVerify(ess_b_i_k, PS_b_i_k, EP_i_k)_.
    -  ph = p;
    -  Return _true_.

## Post-preprocessing

### Computing Commitment Hash

Algorithm _CommitmentHash(p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-   Let _BA[]_ be an empty byte array.
-   Extend _BA[] with bytes(p)_.
-   Extend _BA[] with bytes(m) if exists_.
-   Extend _BA[] with cbytes(GK) if exists_.
-   For each _pkg_i_ in _PKG_p_:
    -   Let PK_i, _, PS_h_i[1..n], _, _, PS_b_i[1..n], _ = pkg_i_.
        For _k = 0 .. n_:
        -   Extend _BA[] with cbytes(PK_i)_.
        -   Extend _BA[] with cbytes(PS_h_i[k])_.
        -   Extend _BA[] with cbytes(PS_b_i[k])_.
- Return _H(bytes(BA[]))_.

### Computing Binding Factors

Algorithm _BindingFactors(p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-  Let _ch = CommitmentHash(p, PKG_p, Optinal m, Optional GK)_.
-  Let binding factors _bf[]_ be an empty list holding binding factors with length _m_.
-  For _i = 1 .. m_:
    -  Let _pkg_i = PKG_p[m]_.
    -  Let _bf_i = H(bytes(i) || bytes(ch))_.
    -  Insert _bf_i_ into _BF[]_.
-  Return _bf[1..m]_.

### Computing Hiding Group Point

Algorithm _HidingGroupPoint(p, PKG_p[1..m])_:
-   Inputs:
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
-  Let hiding point = P_h = a point at infinity.
-  For _i = 1 .. m_:
    -  Let _pkg_i = PKG_p[m]_.
    -   Let _, _, _, COM_h_i[1..t], _, _, _ = pkg_i_.
    -   _P_h += COM_h_i[0]_.
-  Return _P_h_.

### Computing Post Binding Group Point

Algorithm _PostBindingGroupPoint(p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-  Let binding factors _bf = BindingFactors(p, PKG_p, Optional m, Optional GK)_.
-  Let binding point P_b = a point at infinity.
-  For _i = 1 .. m_:
    -  Let _pkg_i = PKG_p[m]_.
    -   Let _, _, _, _, _, _, _COM_b_i[1..t]_ = pkg_i_.
    -   _P_b += COM_b_i[0] • bf[m]_.
-  Return _P_b_.
    
### Computing Group Point

Algorithm _GroupPoint(p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-   Let hiding group point _HGP = HidingGroupPoint(p, PKG_p)_.
-   Let post binding group point _PBGP = PostBindingGroupPoint(p, PKG_p, Optional m, Optional GK)_.
-   Let group point _GP = HGP + PBGP_.
-   Fail if _GP_ is at infinity.
-   Return _GP_.

### Computing Combined Secret Share

Algorithm _CombinedSecretShare(sk, PK, PK[1..n], p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   Secret key of the signatory: _sk_: a secp scalar.
    -   Public key of the signatory: _PK_: a secp point.
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-   Let binding factors _bf[1..m] = BindingFactors(p, PKG_p, Optional m, Optional GK)_.
-   Let _idx_ be the index of _PK_ in _PK[1..n]_.
-   Let _hss_ be the combined hiding secret share set to 0.
-   Let _pbss_ be the combined post binding secret share set to 0.
-   For _i = 0 .. m_:.
    -  Let _, _pkg_i_ = _PKG_p[i]_.
    -  Let PK_i, ess_h_i[1..n], _, _, ess_b_i[1..n], _, _ = pkg_i_.
    -  Let _es = EncryptionKey(sk, PK_i)_.
    -  Let _ess_h = ess_h_i[idx]_.
    -  _hss += ShareDecrypt(ess_h, es)_;
    -  _pbss += ShareDecrypt(ess_b, es) * bf[m]_.
-   Return _hss, pbss_.

### Computing Combined Public Share

Algorithm _CombinedPublicShare(PK, PK[1..n], p, PKG_p[1..m], Optional m, Optional GK)_:
-   Inputs:
    -   Public key of the signatory: _PK_: a secp point.
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
    -   Optional message _m_: a secp scalar.
    -   Optional group key _GK_: a secp point.
-   Let binding factors _bf[1..m] = BindingFactors(p, PKG_p, Optional m, Optional GK)_.
-   Let _idx_ be the index of _PK_ in _PK[1..n]_.
-   Let combined hiding public share _HPS_ = a point at infinity.
-   Let combined post binding public share _PBPS_ = a point at infinity.
-   For _i = 0 .. m_:.
    -  Let _, _pkg_i_ = _PKG_p[i]_.
    -  Let _, _, PS_h_i[1..n], _, _, PS_b_i[1..n], _ = pkg_i_.
    -  _HPS += PS_h_i[idx]_;
    -  _PBPS += PS_b_i[idx] • bf[m]_.
-   Return _HPS, PBPS_.

## Signing

### Partial Signing

Algorithm _PartialSign(sk, PK, m, PK[1..n], p, PKG_p[1..m])_:
-   Inputs:
    -   Secret key of the signatory: _sk_: a secp scalar.
    -   Public key of the signatory: _PK_: a secp point.
    -   Message _m_: a secp scalar.
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
-   Let group key _GK = GroupPoint(0, PKG_0)_.
-   Let group nonce _GN = GroupPoint(p, PKG_p)_.
-   Let challenge _e = H(H("BIP0340/challenge") || H("BIP0340/challenge") + GN || GK || m)_.
-   Let secret key share _d = CombinedSecretShare(sk, PK, PK[1..n], 0, PKG_0, -, -)_.
-   Let secret nonce share _k = CombinedSecretShare(sk, PK, PK[1..n], p, PKG_p, m, GK)_.
-   Let partial signature _s = k + e * d_.
-   Return _s_.

### Partial Signature Verification

Algorithm _PartialVerify(PK, m, s, PK[1..n], p, PKG_p[1..m])_:
-   Inputs:
    -   Public key of the signatory: _PK_: a secp point.
    -   Message _m_: a secp scalar.
    -   Partial signature of the signatory: _ss_: a secp scalar.
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p[1..m]_: a list of DKG packages with length _m_.
-   Let _idx_ be the index of _PK_ in _PK[1..n]_.
-   Let group key _GK = ComputeGroupPoint(GroupPoint(0, PKG_0), -, -)_.
-   Let group nonce _GN = ComputeGroupPoint(GroupPoint(p, PKG_p), m, GK)_.
-   Let challenge _e = H(H("BIP0340/challenge") || H("BIP0340/challenge") + GN || GK || m)_.
-   Let public key share _PKS = CombinedPublicShare(PK, PK[1..n], 0, PKG_0, -, -)_.
-   Let public nonce share _PNS = CombinedPublicShare(PK, PK[1..n], p, PKG_p, m, GK)_.
-   Return _s • G == PNS + (e • PKS)_.

### Signature Aggregation

Algorithm _SigAgg(PK[1..m], m, s[1..m], PK[1..n], p, PKG_p)_:
-   Inputs:
    -   List of signatories who has produced partial signatures _PK[1..m]_: list of secp points.
    -   Message _m_: a secp scalar.
    -   List of partial signatues _s[1..m]_: list of secp scalars. 
    -   Signatory list _PK[1..n]_: list of secp points.
    -   DKG package index _p_: an integer.
    -   DKG package list _PKG_p_: a list of DKG packages with length _m_.
-  Let lagrance index list _li[1..m] = LagranceIndexList(PK[1..m], N[1..n])_.
-  Let aggregate signature _agg = 0_.
-  For _i = 1 .. m_:
    -  Let _PK_i = PK[i]_.
    -  Let partial signature _s_i = s[i]_.
    -  Let lagrance index _li_i = LagranceIndex(PK_i, PK[1..n])_.
    -  Let interpolating value _iv_i = InterpolatingValue(li, li[1..n])_.
    -  Let post-lagrance partial signature _ls_i = s_i * iv_i_.
    -  _agg += ls_i_.
-   Return _agg_.