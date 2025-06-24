use crate::constructive::entity::account::account::Account;

pub fn allowance(_msg_sender: Account) -> bool {
    let is_even: bool = _msg_sender.key().parity().into();
    if !is_even {
        return false;
    }

    if is_blacklisted(_msg_sender) {
        return false;
    }

    if !entry(_msg_sender) {
        return false;
    }

    true
}

// TODO: check for blacklist
pub fn is_blacklisted(_msg_sender: Account) -> bool {
    false
}

// TODO: freemium & CUBICs
pub fn entry(_msg_sender: Account) -> bool {
    true
}
