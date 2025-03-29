pub mod mode;
pub mod session;
pub mod sync;

/// Chain type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Chain {
    Signet,
    Mainnet,
}

impl ToString for Chain {
    fn to_string(&self) -> String {
        match self {
            Chain::Signet => "signet".to_string(),
            Chain::Mainnet => "mainnet".to_string(),
        }
    }
}

/// Operating mode type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    Node,
    Operator,
    Coordinator,
}
