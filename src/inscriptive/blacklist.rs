use crate::{constructive::entity::account::account::Account, operative::Chain};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Guarded blacklist directory.
#[allow(non_camel_case_types)]
pub type BLIST_DIRECTORY = Arc<Mutex<BlacklistDirectory>>;

/// Unix timestamp which a particular account is banned until.
type BlameCounter = u16;
type BlacklistedUntil = u64;

// Initial blame window period is 5 seconds.
const BASE_WAITING_WINDOW: u64 = 5;

/// Directory for the coordinator to manage account blacklists.
pub struct BlacklistDirectory {
    // In-memory list.
    list: HashMap<Account, (BlameCounter, BlacklistedUntil)>,
    // In-storage db.
    db: sled::Db,
}

impl BlacklistDirectory {
    pub fn new(chain: Chain) -> Option<BLIST_DIRECTORY> {
        let path = format!("{}/{}/{}", "db", chain.to_string(), "coordinator/dir/blist");
        let db = sled::open(path).ok()?;

        let mut list = HashMap::<Account, (BlameCounter, BlacklistedUntil)>::new();

        for lookup in db.iter() {
            if let Ok((key, val)) = lookup {
                let account: Account = serde_json::from_slice(&key).ok()?;
                let (blame_counter, blacklisted_until) = serde_json::from_slice(&val).ok()?;

                list.insert(account, (blame_counter, blacklisted_until));
            }
        }

        let blaming_dir = BlacklistDirectory { list, db };

        Some(Arc::new(Mutex::new(blaming_dir)))
    }

    /// Inserts or updates blaming records for a given accounts.
    fn update(
        &mut self,
        account: Account,
        blame_counter: BlameCounter,
        blacklisted_until: BlacklistedUntil,
    ) {
        // Update in-memory
        self.list
            .insert(account, (blame_counter, blacklisted_until));

        // Update in-storage
        {
            let key_serialized = account.serialize();

            let value_serialized = match serde_json::to_vec(&(blame_counter, blacklisted_until)) {
                Ok(bytes) => bytes,
                Err(_) => vec![],
            };

            let _ = self.db.insert(key_serialized, value_serialized);
        }
    }

    /// Blames an account.
    pub fn blame(&mut self, account: Account) {
        match self.list.get(&account) {
            Some((counter, until)) => {
                let mut blame_counter: u16 = counter.to_owned();
                let mut blacklisted_until: u64 = until.to_owned();

                if blame_counter < u16::MAX {
                    blame_counter = blame_counter + 1;

                    let new_blacklisted_until = current_unix_timestamp()
                        + BASE_WAITING_WINDOW
                        + (2 as u64).pow(blame_counter as u32);

                    if new_blacklisted_until > blacklisted_until {
                        blacklisted_until = new_blacklisted_until;
                    }
                } else if blame_counter == u16::MAX {
                    // Permaban
                    blacklisted_until = u64::MAX;
                }

                self.update(account, blame_counter, blacklisted_until);
            }
            None => {
                let blame_counter: u16 = 1;
                let blamed_until: u64 = current_unix_timestamp() + BASE_WAITING_WINDOW;

                self.update(account, blame_counter, blamed_until);
            }
        };
    }

    /// Checks whether an account is blacklisted. Returns the timestamp if any.
    pub fn check_blacklist(&self, account: Account) -> Option<u64> {
        match self.list.get(&account) {
            Some((_, blacklisted_until)) => {
                match blacklisted_until.to_owned() > current_unix_timestamp() {
                    true => Some(blacklisted_until.to_owned()),
                    false => None,
                }
            }
            None => None,
        }
    }

    pub fn manual_blacklist(&mut self, account: Account, blacklisted_until: BlacklistedUntil) {
        let blame_counter = {
            match self.list.get(&account) {
                Some((blame_counter, _)) => blame_counter.to_owned(),
                None => 1,
            }
        };

        self.update(account, blame_counter, blacklisted_until);
    }
}

fn current_unix_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
