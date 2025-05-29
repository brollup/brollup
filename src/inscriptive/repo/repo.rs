use super::repo_error::RepoConstructionError;
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
pub type PROGRAMS_REPO = Arc<Mutex<ProgramsRepo>>;

/// Contract ID.
#[allow(non_camel_case_types)]
type CONTRACT_ID = [u8; 32];

/// Directory for storing contract programs.
pub struct ProgramsRepo {
    programs: HashMap<CONTRACT_ID, Program>,
    programs_db: sled::Db,
}

impl ProgramsRepo {
    pub fn new(chain: Chain) -> Result<PROGRAMS_REPO, RepoConstructionError> {
        // Open the programs db.
        let programs_db = {
            let path = format!("{}/{}/{}", "db", chain.to_string(), "repo");
            sled::open(path).map_err(|e| RepoConstructionError::DBOpenError(e))?
        };

        // Initialize the in-memory list of programs.
        let mut programs = HashMap::<CONTRACT_ID, Program>::new();

        // Collect the in-memory list of programs.
        for (index, lookup) in programs_db.iter().enumerate() {
            if let Ok((key, val)) = lookup {
                // Key is the 32-byte contract id.
                let contract_id: [u8; 32] = key
                    .as_ref()
                    .try_into()
                    .map_err(|_| RepoConstructionError::DBIterCollectInvalidKeyAtIndex(index))?;

                // Deserialize the program bytecode from value.
                let program_bytes: Vec<u8> = val.as_ref().to_vec();

                // Decompile the program from bytecode.
                let program: Program =
                    Program::decompile(&mut program_bytes.into_iter()).map_err(|e| {
                        RepoConstructionError::ProgramDecompileErrorAtKey(contract_id, e)
                    })?;

                // Insert into the in-memory programs list.
                programs.insert(contract_id, program);
            }
        }

        // Construct the repo.
        let repo = ProgramsRepo {
            programs,
            programs_db,
        };

        // Return the guarded repo.
        Ok(Arc::new(Mutex::new(repo)))
    }
}
