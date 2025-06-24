use crate::constructive::entry::combinator::combinators::call::call::Call;

use super::{
    combinator_type::CombinatorType,
    combinators::{
        add::Add, claim::Claim, deploy::Deploy, liftup::liftup::Liftup, r#move::Move,
        recharge::Recharge, reserved::Reserved, revive::Revive, sub::Sub, swapout::Swapout,
    },
};
use serde::{Deserialize, Serialize};

/// The combinator.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Combinator {
    Liftup(Liftup),
    Recharge(Recharge),
    Move(Move),
    Call(Call),
    Add(Add),
    Sub(Sub),
    Deploy(Deploy),
    Swapout(Swapout),
    Revive(Revive),
    Claim(Claim),
    Reserved(Reserved),
}

impl Combinator {
    /// Create a new `Liftup` combinator.
    pub fn new_liftup(liftup: Liftup) -> Combinator {
        Combinator::Liftup(liftup)
    }

    /// Create a new `Recharge` combinator.
    pub fn new_recharge(recharge: Recharge) -> Combinator {
        Combinator::Recharge(recharge)
    }

    /// Create a new `Move` combinator.
    pub fn new_move(r#move: Move) -> Combinator {
        Combinator::Move(r#move)
    }

    /// Create a new `Call` combinator.
    pub fn new_call(call: Call) -> Combinator {
        Combinator::Call(call)
    }

    /// Create a new `Add` combinator.
    pub fn new_add(add: Add) -> Combinator {
        Combinator::Add(add)
    }

    /// Create a new `Sub` combinator.
    pub fn new_sub(sub: Sub) -> Combinator {
        Combinator::Sub(sub)
    }

    /// Create a new `Deploy` combinator.
    pub fn new_deploy(deploy: Deploy) -> Combinator {
        Combinator::Deploy(deploy)
    }

    /// Create a new `Swapout` combinator.
    pub fn new_swapout(swapout: Swapout) -> Combinator {
        Combinator::Swapout(swapout)
    }

    /// Create a new `Revive` combinator.
    pub fn new_revive(revive: Revive) -> Combinator {
        Combinator::Revive(revive)
    }

    /// Create a new `Claim` combinator.
    pub fn new_claim(claim: Claim) -> Combinator {
        Combinator::Claim(claim)
    }

    /// Create a new `Reserved` combinator.
    pub fn new_reserved(reserved: Reserved) -> Combinator {
        Combinator::Reserved(reserved)
    }

    /// Returns the type of the combinator.
    pub fn combinator_type(&self) -> CombinatorType {
        match self {
            Combinator::Liftup(_) => CombinatorType::Liftup,
            Combinator::Recharge(_) => CombinatorType::Recharge,
            Combinator::Move(_) => CombinatorType::Move,
            Combinator::Call(_) => CombinatorType::Call,
            Combinator::Add(_) => CombinatorType::Add,
            Combinator::Sub(_) => CombinatorType::Sub,
            Combinator::Deploy(_) => CombinatorType::Deploy,
            Combinator::Swapout(_) => CombinatorType::Swapout,
            Combinator::Revive(_) => CombinatorType::Revive,
            Combinator::Claim(_) => CombinatorType::Claim,
            Combinator::Reserved(_) => CombinatorType::Reserved,
        }
    }

    /// Serializes the combinator.
    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }
}
