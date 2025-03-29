// Communicative
#[path = "communicative/mod.rs"]
pub mod communicative;

// Constructive
#[path = "constructive/mod.rs"]
pub mod constructive;

// Executive
#[path = "executive/mod.rs"]
pub mod executive;

// Inscriptive
#[path = "inscriptive/mod.rs"]
pub mod inscriptive;

// Operative
#[path = "operative/mod.rs"]
pub mod operative;

// Transmutive
#[path = "transmutive/mod.rs"]
pub mod transmutive;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    Node,
    Operator,
    Coordinator,
}
