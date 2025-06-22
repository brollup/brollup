use crate::constructive::entry::combinator::combinators::deploy::Deploy;
use crate::constructive::entry::combinator::combinators::swapout::Swapout;
use serde::{Deserialize, Serialize};

/// The lower left branch of an entry. Descend from the right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LowerLeftBranch {
    Deploy(Deploy),
    Swapout(Swapout),
}
