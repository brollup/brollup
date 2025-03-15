use crate::{valtype::account::Account, Network, ACCOUNT_REGISTERY};
use secp::Point;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type RegisteryIndex = u32;

/// Directory for the account registeries.
pub struct AccountRegistery {
    network: Network,
    // In-memory list.
    accounts: HashMap<RegisteryIndex, Account>,
    // In-storage db.
    db: sled::Db,
}

impl AccountRegistery {
    pub fn new(network: Network) -> Option<ACCOUNT_REGISTERY> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "registery/account");
        let db = sled::open(path).ok()?;

        let mut accounts = HashMap::<RegisteryIndex, Account>::new();

        for lookup in db.iter() {
            if let Ok((_, val)) = lookup {
                let account: Account = serde_json::from_slice(&val).ok()?;
                let registery_index = account.registery_index()?;
                accounts.insert(registery_index, account);
            }
        }

        let registery = AccountRegistery {
            network,
            accounts,
            db,
        };

        Some(Arc::new(Mutex::new(registery)))
    }

    /// Returns true if the given key is registered.
    pub fn is_registered(&self, key: Point) -> bool {
        self.account_by_key(key).is_some()
    }

    /// Returns the account by the given key.
    pub fn account_by_key(&self, key: Point) -> Option<Account> {
        self.accounts
            .iter()
            .find(|(_, account)| account.key() == key)
            .map(|(_, account)| account.clone())
    }

    /// Returns the account by the given registery index.
    pub fn account_by_index(&self, index: u32) -> Option<Account> {
        self.accounts.get(&index).map(|account| account.clone())
    }

    /// Returns the registery index by the given key.
    pub fn index_by_key(&self, key: Point) -> Option<u32> {
        self.accounts
            .iter()
            .find(|(_, account)| account.key() == key)
            .map(|(index, _)| *index)
    }

    /// Returns the registery index by the given account.
    pub fn index_by_account(&self, account: Account) -> Option<u32> {
        let key = account.key();
        self.index_by_key(key)
    }

    /// Returns the height registery index of the registery.
    fn index_height(&self) -> u32 {
        self.accounts.keys().max().unwrap_or(&0).to_owned()
    }

    /// Returns the next registery index in line.
    fn next_index(&self) -> u32 {
        self.index_height() + 1
    }

    /// Inserts the given account into the registery.
    pub fn insert(&mut self, key: Point) -> bool {
        // Check key parity.
        let is_odd: bool = key.parity().into();
        if is_odd {
            return false;
        }

        // Check if already registered.
        if self.account_by_key(key).is_some() {
            return false;
        }

        let registery_index = self.next_index();
        let account = match Account::new(key, Some(registery_index)) {
            Some(account) => account,
            None => return false,
        };

        // Insert in-memory.
        self.accounts.insert(registery_index, account);

        // Insert in-storage.
        match self
            .db
            .insert(account.key().serialize_xonly(), account.serialize())
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
