use crate::baked;

pub fn provider_list() -> Vec<[u8; 32]> {
    // TODO..
    baked::OPERATOR_SET.to_vec()
}

pub fn is_valid_subset(keys: &Vec<[u8; 32]>) -> bool {
    let provider_list = provider_list();

    for key in keys {
        if !provider_list.contains(key) {
            return false;
        }
    }

    true
}

pub fn is_provider(key: [u8; 32]) -> bool {
    let provider_list = provider_list();
    provider_list.contains(&key)
}
