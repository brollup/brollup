use super::lp::LP;
use crate::{into::IntoPointVec, valtype::account::Account, Network, LP_DIRECTORY};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Directory for the liquidity providers.
pub struct LPDirectory {
    network: Network,
    // In-memory list.
    lps: Vec<LP>,
    // In-storage db.
    db: sled::Db,
}

impl LPDirectory {
    pub fn new(network: Network) -> Option<LP_DIRECTORY> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "dir/lp");
        let db = sled::open(path).ok()?;

        let mut lps = Vec::<LP>::new();

        for lookup in db.iter() {
            if let Ok((_, val)) = lookup {
                let lp: LP = serde_json::from_slice(&val).ok()?;

                lps.push(lp);
            }
        }

        let lp_dir = LPDirectory { network, lps, db };

        Some(Arc::new(Mutex::new(lp_dir)))
    }

    pub fn is_valid_subset(&self, keys: &Vec<[u8; 32]>) -> bool {
        let key_vec = match keys.into_point_vec() {
            Ok(vec) => vec,
            Err(_) => return false,
        };

        for key in key_vec.iter() {
            let account = match Account::new(key.to_owned(), None) {
                Some(account) => account,
                None => return false,
            };

            if let None = self.lp(account) {
                return false;
            }
        }

        true
    }

    pub fn lp(&self, account: Account) -> Option<LP> {
        match self.lps.iter().find(|lp| lp.account() == account).cloned() {
            Some(lp) => match lp.empty_liquidity() {
                true => None,
                false => Some(lp),
            },
            None => None,
        }
    }

    pub fn lp_rank_list(&self, top: u64) -> Vec<LP> {
        let mut ranked_lps = self.lps.clone();
        ranked_lps.sort_by(|a, b| b.lp().cmp(&a.lp()));
        ranked_lps.into_iter().take(top as usize).collect()
    }

    pub fn lp_rank_key_list(&self, top: u64) -> Vec<[u8; 32]> {
        self.lp_rank_list(top)
            .iter()
            .map(|lp| lp.account().key().serialize_xonly())
            .collect()
    }

    pub fn liquidity_add(&mut self, account: Account, liquidity_add: u64, lp_add: u64) -> bool {
        let mut lp = self
            .lps
            .iter()
            .find(|lp| lp.account() == account)
            .cloned()
            .unwrap_or_else(|| LP::new(account, 0, 0));

        lp.update_add(liquidity_add, lp_add)
    }

    pub fn liquidity_remove(
        &mut self,
        account: Account,
        liquidity_remove: u64,
        lp_remove: u64,
    ) -> bool {
        let mut lp = self
            .lps
            .iter()
            .find(|lp| lp.account() == account)
            .cloned()
            .unwrap_or_else(|| LP::new(account, 0, 0));

        lp.update_remove(liquidity_remove, lp_remove)
    }
}
