use super::state_error::{StateConstructionError, StateInsertionError};
use crate::operative::Chain;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Contract ID: 32-byte unique identifier.
#[allow(non_camel_case_types)]
type CONTRACT_ID = [u8; 32];

/// State key.
#[allow(non_camel_case_types)]
type STATE_KEY = Vec<u8>;

/// State value.
#[allow(non_camel_case_types)]
type STATE_VALUE = Vec<u8>;

/// Guarded programs state.
#[allow(non_camel_case_types)]
pub type PROGRAMS_STATE = Arc<Mutex<ProgramsState>>;

/// In-memory and on-disk programs state.
pub struct ProgramsState {
    /// In-memory cache: CONTRACT_ID -> { STATE_KEY -> STATE_VALUE }
    pub states: HashMap<CONTRACT_ID, HashMap<STATE_KEY, STATE_VALUE>>,
    /// Sled DB with contract trees.
    pub states_db: sled::Db,
}

// TODO: Implement a rank-based caching mechanism to only cache the high-ranked program states.
// Right now, we are caching *ALL* contract states in memory.
impl ProgramsState {
    /// Initialize the state for the given chain
    pub fn new(chain: Chain) -> Result<PROGRAMS_STATE, StateConstructionError> {
        // Open the main state db.
        let path = format!("db/{}/state", chain.to_string());
        let states_db = sled::open(path).map_err(StateConstructionError::MainDBOpenError)?;

        // Initialize the in-memory cache of contract states.
        let mut states = HashMap::<CONTRACT_ID, HashMap<STATE_KEY, STATE_VALUE>>::new();

        // Iterate over all contract trees in the main state db.
        for tree_name in states_db.tree_names() {
            let contract_id: [u8; 32] = tree_name
                .as_ref()
                .try_into()
                .map_err(|_| StateConstructionError::InvalidContractIDBytes(tree_name.to_vec()))?;

            // Open the contract tree.
            let tree = states_db
                .open_tree(&tree_name)
                .map_err(|e| StateConstructionError::SubDBOpenError(contract_id, e))?;

            // Iterate over all items in the contract tree.
            let contract_state = tree
                .iter()
                .filter_map(|res| res.ok())
                .map(|(k, v)| (k.to_vec(), v.to_vec()))
                .collect::<HashMap<STATE_KEY, STATE_VALUE>>();

            // Insert the contract state into the in-memory cache.
            states.insert(contract_id, contract_state);
        }

        // Return the guarded programs state.
        Ok(Arc::new(Mutex::new(ProgramsState { states, states_db })))
    }

    /// Get the value by key and contract ID.
    pub fn get_value(&self, key: &STATE_KEY, contract_id: &CONTRACT_ID) -> Option<&STATE_VALUE> {
        self.states
            .get(contract_id)
            .and_then(|state| state.get(key))
    }

    /// Insert a value by key and contract ID.
    pub fn insert_value(
        &mut self,
        key: &STATE_KEY,
        contract_id: &CONTRACT_ID,
        value: &STATE_VALUE,
    ) -> Result<(), StateInsertionError> {
        // In-memory insertion.
        {
            // Get the in-memory contract states.
            let contract_states = self
                .states
                .get_mut(contract_id)
                .ok_or(StateInsertionError::ContractStatesNotFound(*contract_id))?;

            // Insert the value into the in-memory contract states.
            contract_states.insert(key.clone(), value.clone());
        }

        // On-disk insertion.
        {
            // Insert the value into the on-disk contract tree.
            let tree = self
                .states_db
                .open_tree(contract_id)
                .map_err(|e| StateInsertionError::OpenTreeError(*contract_id, e))?;

            // Insert the value into the on-disk contract tree.
            tree.insert(key, value.clone()).map_err(|e| {
                StateInsertionError::ValueInsertError(*contract_id, key.clone(), value.clone(), e)
            })?;
        }

        Ok(())
    }
}
