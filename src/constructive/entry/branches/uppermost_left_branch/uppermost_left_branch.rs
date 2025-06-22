use crate::constructive::entry::combinator::combinators::liftup::Liftup;
use crate::constructive::entry::combinator::combinators::recharge::Recharge;
use serde::{Deserialize, Serialize};

/// The uppermost left branch of an entry.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UppermostLeftBranch {
    pub liftup: Option<Liftup>,
    pub recharge: Option<Recharge>,
}

impl UppermostLeftBranch {
    pub fn new(liftup: Option<Liftup>, recharge: Option<Recharge>) -> Self {
        Self { liftup, recharge }
    }

    pub fn new_liftup(liftup: Liftup) -> Self {
        Self {
            liftup: Some(liftup),
            recharge: None,
        }
    }

    pub fn new_recharge(recharge: Recharge) -> Self {
        Self {
            liftup: None,
            recharge: Some(recharge),
        }
    }

    pub fn new_liftup_and_recharge(liftup: Liftup, recharge: Recharge) -> Self {
        Self {
            liftup: Some(liftup),
            recharge: Some(recharge),
        }
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }
}
