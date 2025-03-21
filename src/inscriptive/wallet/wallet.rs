use super::{
    lift_wallet::{LiftWallet, LIFT_WALLET},
    vtxo_wallet::{VTXOWallet, VTXO_WALLET},
};
use crate::Network;
use colored::Colorize;
use secp::Point;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type WALLET = Arc<Mutex<Wallet>>;

/// Wallet for storing bare Lift outputs.
pub struct Wallet {
    account_key: Point,
    // Lift wallet.
    lift_wallet: LIFT_WALLET,
    // VTXO wallet.
    vtxo_wallet: VTXO_WALLET,
}

impl Wallet {
    pub fn new(network: Network, account_key: Point) -> Option<WALLET> {
        // Retrieve account key from db.

        let account_path = format!("{}/{}/{}", "db", network.to_string(), "wallet");
        let account_db = sled::open(account_path).ok()?;

        let account_key_from_db = match account_db.get(b"account_key") {
            Ok(account) => match account {
                Some(account) => Point::from_slice(&account).ok()?,
                None => {
                    // Save in db if not exists.
                    account_db
                        .insert(b"account_key", account_key.serialize().to_vec())
                        .ok()?;

                    account_key
                }
            },
            Err(_) => return None,
        };

        // Check if account key is consistent.

        if account_key_from_db != account_key {
            eprintln!(
                "{}\n{}",
                "You entered a different nsec than the one used to create the wallet.".red(),
                "Reset database to prooceed with a new account.".red()
            );
            return None;
        }

        // Construct lift wallet.
        let lift_wallet = LiftWallet::new(network)?;

        // Construct vtxo wallet.
        let vtxo_wallet = VTXOWallet::new(network)?;

        let wallet = Wallet {
            account_key,
            lift_wallet,
            vtxo_wallet,
        };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn account_key(&self) -> Point {
        self.account_key.clone()
    }

    pub fn lift_wallet(&self) -> LIFT_WALLET {
        Arc::clone(&self.lift_wallet)
    }

    pub fn vtxo_wallet(&self) -> VTXO_WALLET {
        Arc::clone(&self.vtxo_wallet)
    }
}
