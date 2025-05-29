use crate::executive::program::compiler::compiler_error::{
    ProgramCompileError, ProgramDecompileError,
};

/// The repo construction error.
#[derive(Debug, Clone)]
pub enum RepoConstructionError {
    DBOpenError(sled::Error),
    DBIterCollectInvalidKeyAtIndex(usize),
    ProgramDecompileErrorAtKey([u8; 32], ProgramDecompileError),
}

/// The repo insert error.
#[derive(Debug, Clone)]
pub enum RepoInsertError {
    ProgramCompileError([u8; 32], ProgramCompileError),
    DBInsertError([u8; 32], sled::Error),
    ContractIdAlreadyExists([u8; 32]),
}
