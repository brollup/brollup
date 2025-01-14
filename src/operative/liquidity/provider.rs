use crate::baked;

pub fn provider_list() -> Vec<[u8; 32]> {
    // TODO..
    baked::OPERATOR_SET.to_vec()
}
