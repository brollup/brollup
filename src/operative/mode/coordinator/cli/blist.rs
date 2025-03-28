use crate::{entity::account::Account, BLIST_DIRECTORY};
use secp::Point;

// blist <account> <until>
pub async fn blist_command(parts: Vec<&str>, blacklist_dir: &BLIST_DIRECTORY) {
    if parts.len() != 3 {
        return eprintln!("Incorrect usage.");
    }

    match parts.get(1) {
        Some(account_str) => {
            let bytes: [u8; 32] = match hex::decode(account_str.to_owned()) {
                Ok(bytes) => match bytes.try_into() {
                    Ok(bytes) => bytes,
                    Err(_) => return eprintln!("Invalid <account>."),
                },
                Err(_) => return eprintln!("Invalid <account>."),
            };

            let key = match Point::from_slice(&bytes) {
                Ok(point) => point,
                Err(_) => return eprintln!("Invalid <account>."),
            };

            let account = match Account::new(key, None, None) {
                Some(account) => account,
                None => return eprintln!("Invalid <account>."),
            };

            match parts.get(2) {
                Some(until_str) => {
                    let blacklisted_until: u64 = match until_str.parse() {
                        Ok(until) => until,
                        Err(_) => return eprintln!("Invalid <until>."),
                    };

                    // Blacklist.
                    {
                        let mut _blacklist_dir = blacklist_dir.lock().await;
                        _blacklist_dir.manual_blacklist(account, blacklisted_until);
                    }
                }
                None => return eprintln!("Invalid <until>."),
            }
        }
        None => return eprintln!("Incorrect usage."),
    }
}
