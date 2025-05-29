use crate::executive::program::compiler::compiler_error::ProgramDecompileError;

/// The repo construction error.
#[derive(Debug, Clone)]
pub enum RepoConstructionError {
    DBOpenError(sled::Error),
    DBIterCollectInvalidKeyAtIndex(usize),
    ProgramDecompileErrorAtKey([u8; 32], ProgramDecompileError),
}
