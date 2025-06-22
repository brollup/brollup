use super::combinator::{
    combinator::Combinator,
    combinators::{
        add::Add, call::Call, liftup::Liftup, r#move::Move, recharge::Recharge, reserved::Reserved,
        sub::Sub, swapout::Swapout,
    },
};
use crate::{
    constructive::{
        entity::account::Account,
        entry::branches::{
            uppermost_left_branch::uppermost_left_branch::UppermostLeftBranch,
            uppermost_right_branch::{
                transactive_branch::transactive_branch::TransactiveBranch,
                upper_right_branch::{
                    liquidity_branch::liquidity_branch::LiquidityBranch,
                    right_branch::{
                        lower_left_branch::lower_left_branch::LowerLeftBranch,
                        lower_right_branch::lower_right_branch::LowerRightBranch,
                        right_branch::RightBranch,
                    },
                    upper_right_branch::UpperRightBranch,
                },
                uppermost_right_branch::UppermostRightBranch,
            },
        },
    },
    transmutative::{
        hash::{Hash, HashTag},
        secp::authenticable::AuthSighash,
    },
};
use serde::{Deserialize, Serialize};

/// The entry represents a transaction, containing one or more combinators.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    account: Account,
    uppermost_left_branch: Option<UppermostLeftBranch>,
    uppermost_right_branch: Option<UppermostRightBranch>,
}

impl Entry {
    /// Creates a new entry.
    fn new(
        account: Account,
        uppermost_left_branch: Option<UppermostLeftBranch>,
        uppermost_right_branch: Option<UppermostRightBranch>,
    ) -> Entry {
        Self {
            account,
            uppermost_left_branch,
            uppermost_right_branch,
        }
    }

    // Liftup and/or recharge ONLY.
    pub fn new_nop(account: Account, liftup: Option<Liftup>, recharge: Option<Recharge>) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() && recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };

        let uppermost_right_branch = None;
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new move entry.
    pub fn new_move(
        account: Account,
        r#move: Move,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };

        let uppermost_right_branch = Some(UppermostRightBranch::TransactiveBranch(
            TransactiveBranch::Move(r#move),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_call(
        account: Account,
        call: Call,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::TransactiveBranch(
            TransactiveBranch::Call(call),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new add entry.
    pub fn new_add(
        account: Account,
        add: Add,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::LiquidityBranch(LiquidityBranch::Add(add)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new sub entry.
    pub fn new_sub(
        account: Account,
        sub: Sub,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::LiquidityBranch(LiquidityBranch::Sub(sub)),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new swapout entry.
    pub fn new_swapout(
        account: Account,
        swapout: Swapout,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::LowerLeftBranch(LowerLeftBranch::Swapout(
                swapout,
            ))),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Creates a new reserved entry.
    pub fn new_reserved(
        account: Account,
        reserved: Reserved,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = {
            if liftup.is_some() || recharge.is_some() {
                Some(UppermostLeftBranch::new(liftup, recharge))
            } else {
                None
            }
        };
        let uppermost_right_branch = Some(UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::LowerRightBranch(
                LowerRightBranch::Reserved(reserved),
            )),
        ));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    /// Returns the account of the entry.
    pub fn account(&self) -> Account {
        self.account.clone()
    }

    /// Returns the liftup of the entry.
    pub fn liftup(&self) -> Option<Liftup> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.liftup()
    }

    /// Returns the recharge of the entry.
    pub fn recharge(&self) -> Option<Recharge> {
        let uppermost_left_branch = match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => uppermost_left_branch,
            None => return None,
        };

        uppermost_left_branch.recharge()
    }

    /// Returns the main combinator of the entry.
    pub fn main_combinator(&self) -> Option<Combinator> {
        let uppermost_right_branch = match &self.uppermost_right_branch {
            Some(uppermost_right_branch) => uppermost_right_branch,
            None => return None,
        };

        Some(uppermost_right_branch.main_combinator())
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Validates the account of the entry.
    pub fn validate_account(&self) -> bool {
        let account = self.account();

        if let None = self.uppermost_left_branch {
            if let None = self.uppermost_right_branch {
                return false;
            }
        }

        if let Some(uppermost_left_branch) = &self.uppermost_left_branch {
            if let Some(liftup) = &uppermost_left_branch.liftup {
                if !liftup.validate_account(account) {
                    return false;
                }
            }

            if let Some(recharge) = &uppermost_left_branch.recharge {
                if !recharge.validate_account(account) {
                    return false;
                }
            }
        }

        if let Some(main_combinator) = self.main_combinator() {
            match main_combinator {
                Combinator::Move(r#move) => {
                    if !r#move.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Call(call) => {
                    if !call.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Add(add) => {
                    if !add.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Sub(sub) => {
                    if !sub.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Deploy(deploy) => {
                    if !deploy.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Swapout(swapout) => {
                    if !swapout.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Revive(revive) => {
                    if !revive.validate_account(account) {
                        return false;
                    }
                }
                Combinator::Claim(claim) => {
                    if !claim.validate_account(account) {
                        return false;
                    }
                }
                // Reserved fails the validation.
                Combinator::Reserved(_) => return false,
                // Main combinator cannot be of liftup or recharge.
                Combinator::Liftup(_) => return false,
                Combinator::Recharge(_) => return false,
            }
        }

        true
    }
}

impl AuthSighash for Entry {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account
        preimage.extend(self.account.key().serialize_xonly());

        match &self.uppermost_left_branch {
            Some(uppermost_left_branch) => {
                preimage.push(0x01);

                match &uppermost_left_branch.liftup {
                    Some(liftup) => {
                        preimage.push(0x01);
                        preimage.extend(liftup.auth_sighash());
                    }
                    None => preimage.push(0x00),
                }

                match &uppermost_left_branch.recharge {
                    Some(recharge) => {
                        preimage.push(0x01);
                        preimage.extend(recharge.auth_sighash());
                    }
                    None => preimage.push(0x00),
                }
            }
            None => preimage.push(0x00),
        }

        match &self.uppermost_right_branch {
            Some(uppermost_right_branch) => {
                preimage.push(0x01);

                match &uppermost_right_branch.main_combinator() {
                    Combinator::Move(r#move) => preimage.extend(r#move.auth_sighash()),
                    Combinator::Call(call) => preimage.extend(call.auth_sighash()),
                    Combinator::Add(add) => preimage.extend(add.auth_sighash()),
                    Combinator::Sub(sub) => preimage.extend(sub.auth_sighash()),
                    Combinator::Deploy(deploy) => preimage.extend(deploy.auth_sighash()),
                    Combinator::Swapout(swapout) => preimage.extend(swapout.auth_sighash()),
                    Combinator::Revive(revive) => preimage.extend(revive.auth_sighash()),
                    Combinator::Claim(claim) => preimage.extend(claim.auth_sighash()),
                    // Reserved is not covered.
                    Combinator::Reserved(_) => return [0xffu8; 32],
                    // Liftup and recharge belong to the uppermost left branch.
                    Combinator::Liftup(_) => return [0xffu8; 32],
                    Combinator::Recharge(_) => return [0xffu8; 32],
                }
            }
            None => preimage.push(0x00),
        }

        preimage.hash(Some(HashTag::SighashEntry))
    }
}
