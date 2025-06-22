use crate::constructive::entry::combinator::combinators::call::Call;
use crate::constructive::entry::combinator::combinators::r#move::Move;
use serde::{Deserialize, Serialize};

/// The transactive branch of an entry. Descend from the uppermost right branch.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactiveBranch {
    Move(Move),
    Call(Call),
}
