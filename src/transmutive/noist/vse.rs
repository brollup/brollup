use std::collections::HashMap;

type SignerKey = [u8; 32];
type VSEKey = [u8; 32];

pub struct KeyMap {
    signer: SignerKey,
    corresponds: HashMap<SignerKey, VSEKey>,
}
