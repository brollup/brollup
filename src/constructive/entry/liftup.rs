use crate::txo::lift::Lift;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Liftup {
    lift_prevtxos: Vec<Lift>,
}

impl Liftup {
    pub fn new(lifts: Vec<Lift>) -> Option<Liftup> {
        let mut lift_prevtxos = Vec::<Lift>::new();

        for lift in lifts.iter() {
            match lift.outpoint() {
                Some(_) => lift_prevtxos.push(lift.to_owned()),
                None => return None,
            }
        }

        let liftup = Liftup { lift_prevtxos };

        Some(liftup)
    }

    pub fn lifts(&self) -> Vec<Lift> {
        self.lift_prevtxos.clone()
    }

    pub fn len(&self) -> usize {
        self.lift_prevtxos.len()
    }
}
