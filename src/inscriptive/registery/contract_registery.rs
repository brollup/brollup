use crate::{entity::contract::Contract, Network};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Guarded registery of contracts.
pub type CONTRACT_REGISTERY = Arc<Mutex<ContractRegistery>>;

/// Registery index of a contract for efficient referencing (from 1 to U32::MAX).
type REGISTERY_INDEX = u32;

/// Call counter of a contract used to rank the top 64 contracts.
type CALL_COUNTER = u64;

/// Rank integer representing the position of a contract in the top 64 list (from 1 to 64).
type RANK = u8;

/// Number of top contracts to rank.
const RANK_BY: u8 = 64;

/// Directory for storing contracts and their call counters.
/// There are two in-memory lists, one by registery index and one by call counter.
/// The call counter is used to rank the top 64 contracts.
pub struct ContractRegistery {
    // In-memory list of contracts by registery index..
    contracts: HashMap<REGISTERY_INDEX, Contract>,
    // In-storage db for storing the contracts.
    contracts_db: sled::Db,
    // In-memory list of call counters.
    call_counters: HashMap<REGISTERY_INDEX, CALL_COUNTER>,
    // In-storage db for storing the call counters.
    call_counters_db: sled::Db,
    // In-memory list of top 64 ranked contracts.
    /// The rank is determined based on the call counter.
    ranked_contracts: HashMap<RANK, Contract>,
}

impl ContractRegistery {
    pub fn new(network: Network) -> Option<CONTRACT_REGISTERY> {
        // Open the contracts db.
        let contracts_db = {
            let path = format!("{}/{}/{}", "db", network.to_string(), "registery/contract");
            sled::open(path).ok()?
        };

        // Initialize the in-memory list of contracts.
        let mut contracts = HashMap::<REGISTERY_INDEX, Contract>::new();

        // Collect the in-memory list of contracts by registery index.
        for lookup in contracts_db.iter() {
            if let Ok((_, val)) = lookup {
                // Key is the 32-byte contract id, but is ignored.
                // Value is the contract serialized in bytes.

                // Deserialize the contract from value.
                let mut contract: Contract = serde_json::from_slice(&val).ok()?;

                // Set the rank to None for now.
                // It will shortly soon be set by `rank_contracts`.
                contract.set_rank(None);

                // Get the registery index.
                let registery_index = contract.registery_index();

                // Insert into the in-memory contracts list.
                contracts.insert(registery_index, contract);
            }
        }

        // Open the call counters db.
        let call_counters_db = {
            let path = format!(
                "{}/{}/{}",
                "db",
                network.to_string(),
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

        // Return the list of top 64 ranked contracts.
        let ranked_contracts = Self::rank_contracts(&mut contracts, &call_counters);

        // Construct the contract registery.
        let registery = ContractRegistery {
            contracts,
            contracts_db,
            call_counters,
            call_counters_db,
            ranked_contracts,
        };

        // Return the contract registery.
        Some(Arc::new(Mutex::new(registery)))
    }

    /// Ranks the top 64 contracts by call counter, if equal by registery index.
    fn rank_contracts(
        contracts: &mut HashMap<REGISTERY_INDEX, Contract>,
        call_counters: &HashMap<REGISTERY_INDEX, CALL_COUNTER>,
    ) -> HashMap<RANK, Contract> {
        // Initialize the ranked contracts list.
        let mut ranked_contracts = HashMap::<RANK, Contract>::new();

        // Collect top 64 contract orders by call counter, if equal by registery index.
        let mut top_call_counters = call_counters.iter().collect::<Vec<_>>();
        top_call_counters.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        top_call_counters.truncate(RANK_BY as usize);

        // Insert the ranked contracts into the ranked contracts list.
        for (rank, (registery_index, _)) in top_call_counters.into_iter().enumerate() {
            if let Some(contract) = contracts.get_mut(registery_index) {
                // Rank is rank index + 1.
                let rank = rank as RANK + 1;

                // Set the rank.
                contract.set_rank(Some(rank));

                // Insert into the ranked contracts list.
                ranked_contracts.insert(rank, contract.to_owned());
            }
        }

        // Return the list of ranked contracts.
        ranked_contracts
    }

    //////// READ OPERATIONS ////////

    pub fn print_ranked_contracts(&self) {
        // Convert the ranked contracts to a vector and sort it by rank.
        let mut ordered_ranked_vector = self.ranked_contracts.iter().collect::<Vec<_>>();
        ordered_ranked_vector.sort_by(|a, b| a.0.cmp(b.0));

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
        let mut contract = self
            .contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(_, contract)| contract.clone())?;

        let registery_index = self.registery_index_by_contract_id(contract_id)?;

        // If the contract is in the top 64, set the rank.
        match self.rank_by_registery_index(registery_index) {
            Some(rank) => contract.set_rank(Some(rank)),
            None => contract.set_rank(None),
        }

        // Return the contract.
        Some(contract)
    }

    /// Returns the contract by the given registery index.
    pub fn contract_by_registery_index(&self, registery_index: u32) -> Option<Contract> {
        // Get the contract by the given registery index.
        let mut contract = self.contracts.get(&registery_index)?.to_owned();

        // If the contract is in the top 64, set the rank index.
        match self.rank_by_registery_index(registery_index) {
            Some(rank) => contract.set_rank(Some(rank)),
            None => contract.set_rank(None),
        }

        // Return the contract.
        Some(contract.to_owned())
    }

    /// Returns the contract by the given rank if the contract is in the top 64.
    pub fn contract_by_rank(&self, rank: RANK) -> Option<Contract> {
        // Get the contract by the given rank.
        let contract = self.ranked_contracts.get(&rank)?.to_owned();

        // Return the contract.
        Some(contract)
    }

    /// Returns the rank by the given contract id if the contract is in the top 64.
    pub fn rank_by_contract_id(&self, contract_id: [u8; 32]) -> Option<RANK> {
        self.ranked_contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(rank, _)| *rank)
    }

    /// Returns the rank by the given registery index.
    pub fn rank_by_registery_index(&self, registery_index: u32) -> Option<RANK> {
        self.ranked_contracts
            .iter()
            .find(|(_, contract)| contract.registery_index() == registery_index)
            .map(|(rank, _)| *rank)
    }

    /// Returns the registery index by the given id.
    pub fn registery_index_by_contract_id(&self, contract_id: [u8; 32]) -> Option<u32> {
        self.contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(index, _)| *index)
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

    /// Updates the ranked contracts.
    fn update_ranked_contracts(&mut self) {
        self.ranked_contracts = Self::rank_contracts(&mut self.contracts, &self.call_counters);
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

        // Update the ranked contracts.
        self.update_ranked_contracts();

        true
    }
}
