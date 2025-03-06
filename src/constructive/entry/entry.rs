use crate::hash::{Hash, HashTag};
use crate::{
    combinator::{
        add::Add, call::Call, combinator::Combinator, liftup::Liftup, r#move::Move,
        recharge::Recharge, remove::Remove, reserved::Reserved, swapout::Swapout,
    },
    schnorr::Sighash,
    valtype::account::Account,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UppermostLeftBranch {
    liftup: Option<Liftup>,
    recharge: Option<Recharge>,
}

impl UppermostLeftBranch {
    pub fn new(liftup: Option<Liftup>, recharge: Option<Recharge>) -> Self {
        Self { liftup, recharge }
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.liftup.clone()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.recharge.clone()
    }

    pub fn on(&self) -> bool {
        self.liftup.is_some() || self.recharge.is_some()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Transact {
    Move(Move),
    Call(Call),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Liquidity {
    Add(Add),
    Remove(Remove),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RightBranch {
    Swapout(Swapout),
    Reserved(Reserved),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpperRightBranch {
    Liquidity(Liquidity),
    RightBranch(RightBranch),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UppermostRightBranch {
    Transact(Transact),
    UpperRightBranch(UpperRightBranch),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    account: Account,
    uppermost_left_branch: UppermostLeftBranch,
    uppermost_right_branch: UppermostRightBranch,
}

impl Entry {
    fn new(
        account: Account,
        uppermost_left_branch: UppermostLeftBranch,
        uppermost_right_branch: UppermostRightBranch,
    ) -> Entry {
        Self {
            account,
            uppermost_left_branch,
            uppermost_right_branch,
        }
    }

    pub fn new_move(
        account: Account,
        r#move: Move,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::Transact(Transact::Move(r#move));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_call(
        account: Account,
        call: Call,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::Transact(Transact::Call(call));
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_add(
        account: Account,
        add: Add,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::Liquidity(Liquidity::Add(add)),
        );
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_remove(
        account: Account,
        remove: Remove,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::Liquidity(Liquidity::Remove(remove)),
        );
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_swapout(
        account: Account,
        swapout: Swapout,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::Swapout(swapout)),
        );
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn new_reserved(
        account: Account,
        reserved: Reserved,
        liftup: Option<Liftup>,
        recharge: Option<Recharge>,
    ) -> Entry {
        let uppermost_left_branch = UppermostLeftBranch::new(liftup, recharge);
        let uppermost_right_branch = UppermostRightBranch::UpperRightBranch(
            UpperRightBranch::RightBranch(RightBranch::Reserved(reserved)),
        );
        Self::new(account, uppermost_left_branch, uppermost_right_branch)
    }

    pub fn account(&self) -> Account {
        self.account.clone()
    }

    pub fn liftup(&self) -> Option<Liftup> {
        self.uppermost_left_branch.liftup()
    }

    pub fn recharge(&self) -> Option<Recharge> {
        self.uppermost_left_branch.recharge()
    }

    pub fn main_combinator(&self) -> Combinator {
        match &self.uppermost_right_branch {
            UppermostRightBranch::Transact(transact) => match transact {
                Transact::Move(r#move) => Combinator::Move(r#move.clone()),
                Transact::Call(call) => Combinator::Call(call.clone()),
            },
            UppermostRightBranch::UpperRightBranch(upper_right_branch) => {
                match upper_right_branch {
                    UpperRightBranch::Liquidity(liquidity) => match liquidity {
                        Liquidity::Add(add) => Combinator::Add(add.clone()),
                        Liquidity::Remove(remove) => Combinator::Remove(remove.clone()),
                    },
                    UpperRightBranch::RightBranch(right_branch) => match right_branch {
                        RightBranch::Swapout(swapout) => Combinator::Swapout(swapout.clone()),
                        RightBranch::Reserved(reserved) => Combinator::Reserved(reserved.clone()),
                    },
                }
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn validate_account(&self) -> bool {
        let account = self.account();

        if let Some(liftup) = &self.uppermost_left_branch.liftup {
            if !liftup.validate_account(account) {
                return false;
            }
        }

        if let Some(recharge) = &self.uppermost_left_branch.recharge {
            if !recharge.validate_account(account) {
                return false;
            }
        }

        let main_combinator = self.main_combinator();
        match main_combinator {
            Combinator::Move(r#move) => r#move.validate_account(account),
            Combinator::Call(call) => call.validate_account(account),
            Combinator::Add(add) => add.validate_account(account),
            Combinator::Remove(remove) => remove.validate_account(account),
            Combinator::Swapout(swapout) => swapout.validate_account(account),
            // Reserved fails the validation.
            Combinator::Reserved(_) => return false,
            // Main combinator cannot be of liftup or recharge.
            Combinator::Liftup(_) => return false,
            Combinator::Recharge(_) => return false,
        }
    }
}

impl Sighash for Entry {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account
        preimage.extend(self.account.key().serialize_xonly());

        // Liftup
        match &self.uppermost_left_branch.liftup {
            Some(liftup) => {
                preimage.push(0x01);
                preimage.extend(liftup.sighash());
            }
            None => preimage.push(0x00),
        }

        // Recharge
        match &self.uppermost_left_branch.recharge {
            Some(recharge) => {
                preimage.push(0x01);
                preimage.extend(recharge.sighash());
            }
            None => preimage.push(0x00),
        }

        let main_combinator = self.main_combinator();
        match main_combinator {
            // Unexpected combinator types
            Combinator::Liftup(_) => preimage.extend([0xffu8; 32]),
            Combinator::Recharge(_) => preimage.extend([0xffu8; 32]),
            // Main combinator types
            Combinator::Move(r#move) => preimage.extend(r#move.sighash()),
            Combinator::Call(call) => preimage.extend(call.sighash()),
            Combinator::Add(add) => preimage.extend(add.sighash()),
            Combinator::Remove(remove) => preimage.extend(remove.sighash()),
            Combinator::Swapout(swapout) => preimage.extend(swapout.sighash()),
            Combinator::Reserved(_) => preimage.extend([0xffu8; 32]),
        }

        preimage.hash(Some(HashTag::Sighash))
    }
}
