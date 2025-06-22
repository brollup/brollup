use serde::{Deserialize, Serialize};

/// The type of the combinator.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombinatorType {
    Liftup,
    Recharge,
    Move,
    Call,
    Add,
    Sub,
    Deploy,
    Swapout,
    Revive,
    Claim,
    Reserved,
}

impl CombinatorType {
    /// Returns the string representation of the combinator type.
    pub fn as_str(&self) -> &'static str {
        match self {
            CombinatorType::Liftup => "liftup",
            CombinatorType::Recharge => "recharge",
            CombinatorType::Move => "move",
            CombinatorType::Call => "call",
            CombinatorType::Add => "add",
            CombinatorType::Sub => "sub",
            CombinatorType::Deploy => "deploy",
            CombinatorType::Swapout => "swapout",
            CombinatorType::Revive => "revive",
            CombinatorType::Claim => "claim",
            CombinatorType::Reserved => "reserved",
        }
    }
}
