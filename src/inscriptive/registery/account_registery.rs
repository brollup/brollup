#![allow(non_camel_case_types)]

use crate::{constructive::entity::account::Account, Chain};
use secp::Point;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Guarded registery of accounts.
pub type ACCOUNT_REGISTERY = Arc<Mutex<AccountRegistery>>;

/// Registery index of an account for efficient referencing (from 1 to U32::MAX).
type REGISTERY_INDEX = u32;

/// Call counter of an account used to rank accounts.
type CALL_COUNTER = u64;

/// Rank integer representing the rank position of a contract (from 1 to U32::MAX).
type RANK = u32;

/// Directory for storing accounts and their call counters.
/// There are two in-memory lists, one by registery index and one by call counter.
pub struct AccountRegistery {
    // In-memory list of accounts by rank.
    accounts: HashMap<RANK, Account>,
    // In-storage db for storing the accounts.
    accounts_db: sled::Db,
    // In-memory list of call counters registery index mapping to call counter.
    call_counters: HashMap<REGISTERY_INDEX, CALL_COUNTER>,
    // In-storage db for storing the call counters.
    call_counters_db: sled::Db,
}

impl AccountRegistery {
    pub fn new(chain: Chain) -> Option<ACCOUNT_REGISTERY> {
        // Open the accounts db.
        let accounts_db = {
            let path = format!("{}/{}/{}", "db", chain.to_string(), "registery/account");
            sled::open(path).ok()?
        };

        // Initialize the in-memory list of accounts.
        let mut accounts = HashMap::<REGISTERY_INDEX, Account>::new();

        // Collect the in-memory list of accounts by registery index.
        for (index, lookup) in accounts_db.iter().enumerate() {
            if let Ok((_, val)) = lookup {
                // Key is the 32-byte account id, but is ignored.
                // Value is the account serialized in bytes.

                // Deserialize the account from value.
                let account: Account = serde_json::from_slice(&val).ok()?;

                // Insert into the in-memory accounts list.
                // Set rank to index for now.
                // It will shortly soon be set by `update_accounts_ranks`.
                accounts.insert(index as REGISTERY_INDEX, account);
            }
        }

        // Open the call counters db.
        let call_counters_db = {
            let path = format!(
                "{}/{}/{}",
                "db",
                chain.to_string(),
                "registery/account/counter"
            );

            sled::open(path).ok()?
        };

        // Initialize the in-memory list of call counters.
        let mut call_counters = HashMap::<REGISTERY_INDEX, CALL_COUNTER>::new();

        // Collect the in-memory list of call counters.
        for lookup in call_counters_db.iter() {
            if let Ok((key, val)) = lookup {
                // Key is the 4-byte registery index.
                // Value is the 8-byte call counter.

                // Deserialize the registery index from key.
                let registery_index: REGISTERY_INDEX =
                    u32::from_le_bytes(key.as_ref().try_into().ok()?);

                // Deserialize the call counter from value.
                let call_counter: CALL_COUNTER = u64::from_le_bytes(val.as_ref().try_into().ok()?);

                // Insert into the in-memory call counters list.
                call_counters.insert(registery_index, call_counter);
            }
        }

        // Construct the account registery.
        let mut registery = AccountRegistery {
            accounts,
            accounts_db,
            call_counters,
            call_counters_db,
        };

        // Update the accounts ranks which were initially set to 0.
        if !registery.rank_accounts() {
            return None;
        }

        // Return the account registery.
        Some(Arc::new(Mutex::new(registery)))
    }

    /// Sorts the call counters.
    fn sort_call_counters(
        call_counters: &HashMap<REGISTERY_INDEX, CALL_COUNTER>,
    ) -> Vec<(&REGISTERY_INDEX, &CALL_COUNTER)> {
        // Sort the call counters by call counter, if equal by registery index.
        let mut call_counters_sorted = call_counters.iter().collect::<Vec<_>>();
        call_counters_sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

        // Return the sorted call counters.
        call_counters_sorted
    }

    /// Updates the in-memory list of accounts with their new rank.
    fn rank_accounts(&mut self) -> bool {
        // Sort the call counters.
        let call_counters_sorted = Self::sort_call_counters(&self.call_counters);

        // Initialize the ranked accounts list.
        let mut ranked_accounts = HashMap::<RANK, Account>::new();

        // Insert the ranked contracts into the ranked contracts list.
        for (index, (registery_index, _)) in call_counters_sorted.into_iter().enumerate() {
            // Get the contract by registery index.
            let account = match self.accounts.iter_mut().find(|(_, account)| {
                &account.registery_index() == &Some(registery_index.to_owned())
            }) {
                Some((_, account)) => account,
                None => return false,
            };

            // Rank is index + 1.
            let rank = index as u32 + 1;

            // Set the rank.
            account.set_rank(Some(rank));

            // Insert into the ranked accounts list.
            ranked_accounts.insert(rank, account.to_owned());
        }

        // Update the ranked accounts.
        self.accounts = ranked_accounts;

        true
    }

    //////// READ OPERATIONS ////////

    pub fn print_ranked_accounts(&self) {
        // Convert the ranked accounts to a vector and sort it by rank.
        let mut ordered_ranked_vector = self.accounts.iter().collect::<Vec<_>>();
        ordered_ranked_vector.sort_by(|a, b| a.1.rank().cmp(&b.1.rank()));

        // Print the ranked accounts.
        for (rank, account) in ordered_ranked_vector.iter() {
            let registery_index = match account.registery_index() {
                Some(index) => index,
                None => {
                    eprintln!(
                        "Account {} has no registery index.",
                        hex::encode(account.key().serialize_uncompressed())
                    );
                    return;
                }
            };

            println!(
                "Rank: #{}, Account Key: {}, Registery Index: {}",
                rank,
                hex::encode(account.key().serialize_uncompressed()),
                registery_index
            );
        }
    }

    /// Returns whether the given contract ID is registered.
    pub fn is_registered(&self, key: Point) -> bool {
        self.account_by_key(key).is_some()
    }

    /// Returns the account by the given account key.
    pub fn account_by_key(&self, key: Point) -> Option<Account> {
        let account = self
            .accounts
            .iter()
            .find(|(_, account)| account.key() == key)
            .map(|(_, account)| account.clone())?;

        // Return the account.
        Some(account)
    }

    /// Returns the account by the given account key.
    pub fn account_by_key_maybe_registered(&self, key: Point) -> Option<Account> {
        match self.account_by_key(key) {
            Some(account) => Some(account),
            None => {
                let account = Account::new(key, None, None);
                match account {
                    Some(account) => Some(account),
                    None => None,
                }
            }
        }
    }

    /// Returns the account by the given registery index.
    pub fn account_by_registery_index(&self, registery_index: u32) -> Option<Account> {
        self.accounts
            .iter()
            .find(|(_, account)| account.registery_index() == Some(registery_index))
            .map(|(_, account)| account.to_owned())
    }

    /// Returns the contract by the given rank.
    pub fn account_by_rank(&self, rank: RANK) -> Option<Account> {
        self.accounts.get(&rank).map(|account| account.to_owned())
    }

    /// Returns the current registery index height.
    pub fn registery_index_height(&self) -> u32 {
        self.accounts.keys().max().unwrap_or(&0).to_owned()
    }

    //////// WRITE-UPDATE OPERATIONS ////////

    /// Inserts the given account into the registery.
    fn insert_account(&mut self, key: Point, registery_index: u32) -> bool {
        // Construct the account.
        let account = match Account::new(key, Some(registery_index), None) {
            Some(account) => account,
            None => return false,
        };

        // Insert into the in-memory accounts list.
        self.accounts.insert(registery_index, account);

        // Insert into the in-storage contracts db.
        if let Err(_) = self
            .accounts_db
            .insert(&account.key().serialize_xonly(), account.serialize())
        {
            return false;
        }

        // Initial call counter value is set to zero.
        let initial_call_counter_value: u64 = 0;

        // Insert into the in-memory call counters list.
        self.call_counters
            .insert(registery_index, initial_call_counter_value);

        // Insert into the in-storage call counters db.
        if let Err(_) = self.call_counters_db.insert(
            &registery_index.to_le_bytes(),
            initial_call_counter_value.to_le_bytes().to_vec(),
        ) {
            return false;
        }

        true
    }

    // Increments the call counter for the given contract.
    fn increment_call_counter(&mut self, registery_index: u32, increment_by: u64) -> bool {
        // Update the call counter in-memory, and return the new call counter.
        let new_call_counter = self
            .call_counters
            .entry(registery_index)
            .and_modify(|counter| *counter += increment_by)
            .or_insert(increment_by);

        // Update the call counter in-storage.
        if let Err(_) = self.call_counters_db.insert(
            &registery_index.to_le_bytes(),
            new_call_counter.to_le_bytes().to_vec(),
        ) {
            return false;
        }

        true
    }

    /// Updates the registery in a single batch operation.
    /// This is the only public operation that can be used to write/update the contract registery.
    pub fn batch_update(
        &mut self,
        // List of new contracts IDs to register.
        accounts_to_register: Vec<Point>,
        // List of contracts called and the number of times that they were called.
        accounts_called: HashMap<Account, u64>,
    ) -> bool {
        // Check if all the new contracts are not already registered.
        for account_key in accounts_to_register.iter() {
            if self.is_registered(account_key.to_owned()) {
                return false;
            }
        }

        // Check if all the contracts called are registered.
        for (account, _) in accounts_called.iter() {
            if !self.is_registered(account.key()) {
                return false;
            }
        }

        // Get the current registery index height.
        let mut registery_index_height = self.registery_index_height();

        // Register the new contracts.
        for account_key in accounts_to_register {
            // Increment the registery index height.
            registery_index_height += 1;

            // Insert the account into the registery.
            if !self.insert_account(account_key, registery_index_height) {
                return false;
            }
        }

        // Increment the call counter for the given contracts.
        for (account, num_times_called) in accounts_called {
            // Get the registery index of the account.
            let registery_index = match account.registery_index() {
                Some(index) => index,
                None => return false,
            };

            // Increment the call counter.
            if !self.increment_call_counter(registery_index, num_times_called) {
                return false;
            }
        }

        // Update the accounts ranks.
        if !self.rank_accounts() {
            return false;
        }

        true
    }
}
