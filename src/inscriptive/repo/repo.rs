use crate::{
    executive::program::{compiler::compiler::ProgramCompiler, program::Program},
    operative::Chain,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Guarded repo of contracts.
#[allow(non_camel_case_types)]
pub type CONTRACTS_REPO = Arc<Mutex<ContractsRepo>>;

/// Contract ID.
#[allow(non_camel_case_types)]
type CONTRACT_ID = [u8; 32];

/// Directory for storing contract programs.
pub struct ContractsRepo {
    contracts: HashMap<CONTRACT_ID, Program>,
    contracts_db: sled::Db,
}

impl ContractsRepo {
    pub fn new(chain: Chain) -> Option<CONTRACTS_REPO> {
        // Open the contracts db.
        let contracts_db = {
            let path = format!("{}/{}/{}", "db", chain.to_string(), "repo");
            sled::open(path).ok()?
        };

        // Initialize the in-memory list of contracts.
        let mut contracts = HashMap::<CONTRACT_ID, Program>::new();

        // Collect the in-memory list of contract programs.
        for lookup in contracts_db.iter() {
            if let Ok((key, val)) = lookup {
                // Key is the 32-byte contract id.
                let contract_id: [u8; 32] = key.as_ref().try_into().ok()?;

                // Deserialize the program bytecode from value.
                let program_bytes: Vec<u8> = val.as_ref().to_vec();

                // Decompile the program from bytecode.
                let program: Program = Program::decompile(&mut program_bytes.into_iter()).ok()?;

                // Insert into the in-memory contracts list.
                contracts.insert(contract_id, program);
            }
        }

        // Construct the repo.
        let repo = ContractsRepo {
            contracts,
            contracts_db,
        };

        // Return the guarded repo.
        Some(Arc::new(Mutex::new(repo)))
    }
}
