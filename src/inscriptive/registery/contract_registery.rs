use crate::{constructive::entity::contract::Contract, operative::Chain};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Guarded registery of contracts.
#[allow(non_camel_case_types)]
pub type CONTRACT_REGISTERY = Arc<Mutex<ContractRegistery>>;

/// Registery index of a contract for efficient referencing (from 1 to U32::MAX).
#[allow(non_camel_case_types)]
type REGISTERY_INDEX = u32;

/// Call counter of a contract used to rank contracts.
#[allow(non_camel_case_types)]
type CALL_COUNTER = u64;

/// Rank integer representing the rank position of a contract (from 1 to U32::MAX).
#[allow(non_camel_case_types)]
type RANK = u32;

/// Directory for storing contracts and their call counters.
/// There are two in-memory lists, one by registery index and one by call counter.
pub struct ContractRegistery {
    // In-memory list of contracts by rank.
    contracts: HashMap<RANK, Contract>,
    // In-storage db for storing the contracts.
    contracts_db: sled::Db,
    // In-memory list of call counters registery index mapping to call counter.
    call_counters: HashMap<REGISTERY_INDEX, CALL_COUNTER>,
    // In-storage db for storing the call counters.
    call_counters_db: sled::Db,
}

impl ContractRegistery {
    pub fn new(chain: Chain) -> Option<CONTRACT_REGISTERY> {
        // Open the contracts db.
        let contracts_db = {
            let path = format!("{}/{}/{}", "db", chain.to_string(), "registery/contract");
            sled::open(path).ok()?
        };

        // Initialize the in-memory list of contracts.
        let mut contracts = HashMap::<REGISTERY_INDEX, Contract>::new();

        // Collect the in-memory list of contracts by registery index.
        for (index, lookup) in contracts_db.iter().enumerate() {
            if let Ok((_, val)) = lookup {
                // Key is the 32-byte contract id, but is ignored.
                // Value is the contract serialized in bytes.

                // Deserialize the contract from value.
                let contract: Contract = serde_json::from_slice(&val).ok()?;

                // Insert into the in-memory contracts list.
                // Set rank to index for now.
                // It will shortly soon be set by `update_contracts_ranks`.
                contracts.insert(index as REGISTERY_INDEX, contract);
            }
        }

        // Open the call counters db.
        let call_counters_db = {
            let path = format!(
                "{}/{}/{}",
                "db",
                chain.to_string(),
                "registery/contract/counter"
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

        // Construct the contract registery.
        let mut registery = ContractRegistery {
            contracts,
            contracts_db,
            call_counters,
            call_counters_db,
        };

        // Update the contracts ranks which were initially set to 0.
        if !registery.rank_contracts() {
            return None;
        }

        // Return the contract registery.
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

    /// Updates the in-memory list of contracts with their new rank.
    fn rank_contracts(&mut self) -> bool {
        // Sort the call counters.
        let call_counters_sorted = Self::sort_call_counters(&self.call_counters);

        // Initialize the ranked contracts list.
        let mut ranked_contracts = HashMap::<RANK, Contract>::new();

        // Insert the ranked contracts into the ranked contracts list.
        for (index, (registery_index, _)) in call_counters_sorted.into_iter().enumerate() {
            // Get the contract by registery index.
            let contract = match self
                .contracts
                .iter_mut()
                .find(|(_, contract)| &contract.registery_index() == registery_index)
            {
                Some((_, contract)) => contract,
                None => return false,
            };

            // Rank is index + 1.
            let rank = index as u32 + 1;

            // Set the rank.
            contract.set_rank(Some(rank));

            // Insert into the ranked contracts list.
            ranked_contracts.insert(rank, contract.to_owned());
        }

        // Update the ranked contracts.
        self.contracts = ranked_contracts;

        true
    }

    //////// READ OPERATIONS ////////

    pub fn print_ranked_contracts(&self) {
        // Convert the ranked contracts to a vector and sort it by rank.
        let mut ordered_ranked_vector = self.contracts.iter().collect::<Vec<_>>();
        ordered_ranked_vector.sort_by(|a, b| a.1.rank().cmp(&b.1.rank()));

        // Print the ranked contracts.
        for (rank, contract) in ordered_ranked_vector.iter() {
            println!(
                "Rank: #{}, Contract ID: {}, Registery Index: {}",
                rank,
                hex::encode(contract.contract_id()),
                contract.registery_index()
            );
        }
    }

    /// Returns whether the given contract ID is registered.
    pub fn is_registered(&self, contract_id: [u8; 32]) -> bool {
        self.contract_by_contract_id(contract_id).is_some()
    }

    /// Returns the contract by the given contract id.
    pub fn contract_by_contract_id(&self, contract_id: [u8; 32]) -> Option<Contract> {
        let contract = self
            .contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(_, contract)| contract.clone())?;

        // Return the contract.
        Some(contract)
    }

    /// Returns the contract by the given registery index.
    pub fn contract_by_registery_index(&self, registery_index: u32) -> Option<Contract> {
        self.contracts
            .iter()
            .find(|(_, contract)| contract.registery_index() == registery_index)
            .map(|(_, contract)| contract.to_owned())
    }

    /// Returns the contract by the given rank.
    pub fn contract_by_rank(&self, rank: RANK) -> Option<Contract> {
        self.contracts
            .get(&rank)
            .map(|contract| contract.to_owned())
    }

    /// Returns the current registery index height.
    pub fn registery_index_height(&self) -> u32 {
        self.contracts.keys().max().unwrap_or(&0).to_owned()
    }

    //////// WRITE-UPDATE OPERATIONS ////////

    /// Inserts the given contract into the registery.
    fn insert_contract(&mut self, contract_id: [u8; 32], registery_index: u32) -> bool {
        // Construct the contract.
        let contract = Contract::new(contract_id, registery_index, None);

        // Insert into the in-memory contracts list.
        self.contracts.insert(registery_index, contract);

        // Insert into the in-storage contracts db.
        if let Err(_) = self
            .contracts_db
            .insert(&contract.contract_id(), contract.serialize())
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
        contracts_to_register: Vec<[u8; 32]>,
        // List of contracts called and the number of times that they were called.
        contracts_called: HashMap<Contract, u64>,
    ) -> bool {
        // Check if all the new contracts are not already registered.
        for contract_id in contracts_to_register.iter() {
            if self.is_registered(contract_id.to_owned()) {
                return false;
            }
        }

        // Check if all the contracts called are registered.
        for (contract, _) in contracts_called.iter() {
            if !self.is_registered(contract.contract_id()) {
                return false;
            }
        }

        // Get the current registery index height.
        let mut registery_index_height = self.registery_index_height();

        // Register the new contracts.
        for contract_id in contracts_to_register {
            registery_index_height += 1;
            if !self.insert_contract(contract_id, registery_index_height) {
                return false;
            }
        }

        // Increment the call counter for the given contracts.
        for (contract, num_times_called) in contracts_called {
            if !self.increment_call_counter(contract.registery_index(), num_times_called) {
                return false;
            }
        }

        // Update the contracts ranks.
        if !self.rank_contracts() {
            return false;
        }

        true
    }
}
