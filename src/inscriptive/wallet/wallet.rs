use crate::{
    txo::{lift::Lift, vtxo::VTXO},
    Network, WALLET,
};
use colored::Colorize;
use secp::Point;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wallet for storing bare Lift outputs.
pub struct Wallet {
    account_key: Point,
    // In-memory Lift set.
    lifts: Vec<Lift>,
    // In-storage Lifts db.
    lifts_db: sled::Db,
    // In memory VTXO set.
    vtxos: Vec<VTXO>,
    // In-storage VTXO set.
    vtxos_db: sled::Db,
}

impl Wallet {
    pub fn new(network: Network, account_key: Point) -> Option<WALLET> {
        // Retrieve account key from db.

        let account_path = format!("{}/{}/{}", "db", network.to_string(), "wallet");
        let account_db = sled::open(account_path).ok()?;

        let account_key_from_db = match account_db.get(b"account") {
            Ok(account) => match account {
                Some(account) => Point::from_slice(&account).ok()?,
                None => {
                    // Save in db if not exists.
                    account_db
                        .insert(b"account", account_key.serialize().to_vec())
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

        // Collect lifts from db.

        let lifts_path = format!("{}/{}/{}", "db", network.to_string(), "wallet/lift");
        let lifts_db = sled::open(lifts_path).ok()?;

        let mut lift_set = Vec::<Lift>::new();

        for lookup in lifts_db.iter() {
            if let Ok((_, val)) = lookup {
                if let Ok(lift) = serde_json::from_slice::<Lift>(&val) {
                    lift_set.push(lift);
                }
            }
        }

        // Collect vtxos from db.

        let vtxos_path = format!("{}/{}/{}", "db", network.to_string(), "wallet/vtxo");
        let vtxos_db = sled::open(vtxos_path).ok()?;

        let mut vtxo_set = Vec::<VTXO>::new();

        for lookup in vtxos_db.iter() {
            if let Ok((_, val)) = lookup {
                let vtxo: VTXO = serde_json::from_slice(&val).ok()?;
                vtxo_set.push(vtxo);
            }
        }

        let wallet = Wallet {
            account_key,
            lifts: lift_set,
            lifts_db,
            vtxos: vtxo_set,
            vtxos_db,
        };

        Some(Arc::new(Mutex::new(wallet)))
    }

    pub fn account_key(&self) -> Point {
        self.account_key.clone()
    }

    pub fn lifts(&self) -> Vec<Lift> {
        self.lifts.clone()
    }

    pub fn vtxos(&self) -> Vec<VTXO> {
        self.vtxos.clone()
    }

    /// Inserts a new Lift into the wallet.
    pub fn insert_lift(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.
        if self.lifts.iter().any(|l| l.outpoint() == Some(outpoint)) {
            return false;
        }

        // Insert in-memory.
        self.lifts.push(lift.to_owned());

        // Insert in-db.
        match self
            .lifts_db
            .insert(&outpoint.bytes(), lift.to_owned().serialize())
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove_lift(&mut self, lift: &Lift) -> bool {
        let outpoint = match lift.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .lifts
            .iter()
            .position(|l| l.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.lifts.remove(index);

        // Remove in-db.
        match self.lifts_db.remove(&outpoint.bytes()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Inserts a new Lift into wallet.
    pub fn insert_vtxo(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is an overlap in the set.

        if self
            .vtxos
            .iter()
            .any(|vtxo| vtxo.outpoint() == Some(outpoint))
        {
            return false;
        }

        // Insert in-memory.
        self.vtxos.push(vtxo.to_owned());

        // Insert in-db.
        match self
            .vtxos_db
            .insert(&outpoint.bytes(), vtxo.to_owned().serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Removes a Lift from the wallet.
    pub fn remove_vtxo(&mut self, vtxo: &VTXO) -> bool {
        let outpoint = match vtxo.outpoint() {
            Some(outpoint) => outpoint,
            None => return false,
        };

        // Check if there is one in the set.
        let Some(index) = self
            .vtxos
            .iter()
            .position(|vtxo| vtxo.outpoint() == Some(outpoint))
        else {
            return false;
        };

        // Remove in-memory.
        self.vtxos.remove(index);

        // Remove in-db.
        match self.vtxos_db.remove(&outpoint.bytes()) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
