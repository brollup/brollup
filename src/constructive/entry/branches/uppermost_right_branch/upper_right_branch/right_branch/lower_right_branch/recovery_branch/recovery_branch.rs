use crate::constructive::entry::combinator::combinators::claim::Claim;
use crate::constructive::entry::combinator::combinators::revive::Revive;
use serde::{Deserialize, Serialize};

/// The recovery branch of an entry. Descend from the lower right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecoveryBranch {
    Revive(Revive),
    Claim(Claim),
}
