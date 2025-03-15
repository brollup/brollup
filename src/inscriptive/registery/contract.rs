use crate::{valtype::contract::Contract, Network, CONTRACT_REGISTERY};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type RegisteryIndex = u32;

/// Directory for the contract registeries.
pub struct ContractRegistery {
    // In-memory list.
    contracts: HashMap<RegisteryIndex, Contract>,
    // In-storage db.
    db: sled::Db,
}

impl ContractRegistery {
    pub fn new(network: Network) -> Option<CONTRACT_REGISTERY> {
        let path = format!("{}/{}/{}", "db", network.to_string(), "registery/contract");
        let db = sled::open(path).ok()?;

        let mut contracts = HashMap::<RegisteryIndex, Contract>::new();

        for lookup in db.iter() {
            if let Ok((_, val)) = lookup {
                let contract: Contract = serde_json::from_slice(&val).ok()?;
                let registery_index = contract.registery_index();
                contracts.insert(registery_index, contract);
            }
        }

        let registery = ContractRegistery { contracts, db };

        Some(Arc::new(Mutex::new(registery)))
    }

    /// Returns true if the given account is registered.
    pub fn is_registered(&self, contract_id: [u8; 32]) -> bool {
        self.contract_by_id(contract_id).is_some()
    }

    /// Returns the contract by the given id.
    pub fn contract_by_id(&self, contract_id: [u8; 32]) -> Option<Contract> {
        self.contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(_, contract)| contract.clone())
    }

    /// Returns the contract by the given registery index.
    pub fn contract_by_index(&self, index: u32) -> Option<Contract> {
        self.contracts.get(&index).map(|contract| contract.clone())
    }

    /// Returns the registery index by the given id.
    pub fn index_by_id(&self, contract_id: [u8; 32]) -> Option<u32> {
        self.contracts
            .iter()
            .find(|(_, contract)| contract.contract_id() == contract_id)
            .map(|(index, _)| *index)
    }

    /// Returns the registery index by the given contract.
    pub fn index_by_contract(&self, contract: Contract) -> Option<u32> {
        let contract_id = contract.contract_id();
        self.index_by_id(contract_id)
    }

    /// Returns the height registery index of the registery.
    fn index_height(&self) -> u32 {
        self.contracts.keys().max().unwrap_or(&0).to_owned()
    }

    /// Returns the next registery index in line.
    fn next_index(&self) -> u32 {
        self.index_height() + 1
    }

    /// Inserts the given contract into the registery.
    pub fn insert(&mut self, contract_id: [u8; 32]) -> bool {
        // Check if already registered.
        if self.contract_by_id(contract_id).is_some() {
            return false;
        }

        let registery_index = self.next_index();
        let contract = Contract::new(contract_id, registery_index);

        // Insert in-memory.
        self.contracts.insert(registery_index, contract);

        // Insert in-storage.
        match self.db.insert(contract.contract_id(), contract.serialize()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
