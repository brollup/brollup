use std::collections::HashMap;

use secp::{MaybePoint, Point};
use serde::{Deserialize, Serialize};

use crate::{
    hash::Hash,
    into::{IntoPointByteVec, IntoPointVec, IntoScalar},
    noist::setup::setup::VSESetup,
    schnorr::Authenticable,
};

use super::package::DKGPackage;

#[derive(Clone, Serialize, Deserialize)]
pub struct DKGSession {
    nonce: u64,
    signatories: Vec<Point>,
    packages: HashMap<Point, Authenticable<DKGPackage>>,
}

impl DKGSession {
    pub fn new(nonce: u64, signatories: &Vec<[u8; 32]>) -> Option<Self> {
        let mut signatories = signatories.clone();
        signatories.sort();

        let session = DKGSession {
            nonce,
            signatories: signatories.into_point_vec().ok()?,
            packages: HashMap::<Point, Authenticable<DKGPackage>>::new(),
        };

        Some(session)
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories.clone()
    }

    pub fn auth_packages(&self) -> HashMap<Point, Authenticable<DKGPackage>> {
        self.packages.clone()
    }

    pub fn packages(&self) -> HashMap<Point, DKGPackage> {
        let mut packages = HashMap::<Point, DKGPackage>::new();
        for (signatory, auth_package) in self.packages.iter() {
            packages.insert(signatory.to_owned(), auth_package.object());
        }
        packages
    }

    pub fn ordered_packages(&self) -> Vec<(Point, DKGPackage)> {
        let mut maps: Vec<(Point, DKGPackage)> = self.packages().into_iter().collect();
        maps.sort_by(|a, b| a.0.cmp(&b.0));
        maps
    }

    pub fn insert(
        &mut self,
        auth_package: &Authenticable<DKGPackage>,
        vse_setup: &VSESetup,
    ) -> bool {
        if !auth_package.authenticate() {
            return false;
        }

        let package = auth_package.object();

        let package_signatory = package.signatory();

        if package_signatory.serialize_xonly() != auth_package.key() {
            return false;
        }

        if !self.signatories.contains(&package_signatory) {
            return false;
        }

        if let Some(_) = self.packages.get(&package_signatory) {
            return false;
        }

        let self_signatories = match self.signatories.into_xpoint_vec() {
            Ok(vec) => vec,
            Err(_) => return false,
        };

        if !package.is_complete(&self_signatories) {
            return false;
        }

        if !package.vss_verify() {
            return false;
        }

        if !package.vse_verify(vse_setup) {
            return false;
        }

        self.packages
            .insert(package_signatory, auth_package.to_owned());

        true
    }

    pub fn verify(&self, vse_setup: &VSESetup) -> bool {
        for (signatory, auth_package) in self.packages.iter() {
            if auth_package.key() != signatory.serialize_xonly() {
                return false;
            }

            if !auth_package.authenticate() {
                return false;
            }

            let self_signatories = match self.signatories.into_xpoint_vec() {
                Ok(vec) => vec,
                Err(_) => return false,
            };

            let package = auth_package.object();

            if !package.is_complete(&self_signatories) {
                return false;
            }

            if !package.vss_verify() {
                return false;
            }

            if !package.vse_verify(vse_setup) {
                return false;
            }
        }

        true
    }

    pub fn is_full(&self) -> bool {
        self.signatories.len() == self.packages.len()
    }

    pub fn is_above_threshold(&self) -> bool {
        let threshold = (self.signatories.len() / 2) + 1;
        self.packages.len() >= threshold
    }

    pub fn combined_hiding_point(&self) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages().iter() {
            combined_point = combined_point + package.hiding().constant_point()?;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn combined_pre_binding_point(&self) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages().iter() {
            combined_point = combined_point + package.binding().constant_point()?;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn combined_post_binding_point(
        &self,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;
        let binding_factors = self.binding_factors(group_key, message)?;
        let ordered_packages = self.ordered_packages();

        if binding_factors.len() != ordered_packages.len() {
            return None;
        }

        for (index, (_, package)) in ordered_packages.iter().enumerate() {
            let binding_factor = binding_factors[index].into_scalar().ok()?;
            combined_point =
                combined_point + (package.binding().constant_point()? * binding_factor);
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn combined_full_point(
        &self,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Point> {
        let hiding = self.combined_hiding_point()?;
        let binding = self.combined_post_binding_point(group_key, message)?;
        match hiding + binding {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn group_commitment_hash(&self) -> Option<[u8; 32]> {
        let mut preimage = Vec::<u8>::new();
        preimage.extend(self.nonce.to_be_bytes());

        for (signatory, package) in self.ordered_packages().iter() {
            preimage.extend(signatory.serialize_xonly());
            preimage.extend(package.hiding().constant_point()?.serialize_uncompressed());
            preimage.extend(package.binding().constant_point()?.serialize_uncompressed());
        }

        Some(preimage.hash(Some(crate::hash::HashTag::GroupCommitment)))
    }

    pub fn binding_factors(
        &self,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Vec<[u8; 32]>> {
        let mut binding_factors = Vec::<[u8; 32]>::new();

        for (index, _) in self.packages.iter().enumerate() {
            let mut preimage = Vec::<u8>::new();

            preimage.extend(index.to_be_bytes());
            preimage.extend(self.group_commitment_hash()?);

            if let Some(group_key) = group_key {
                preimage.extend(group_key);
            };

            if let Some(message) = message {
                preimage.extend(message);
            };

            let binding_factor = preimage.hash(Some(crate::hash::HashTag::BindingFactor));
            binding_factors.push(binding_factor);
        }
        Some(binding_factors)
    }
}
