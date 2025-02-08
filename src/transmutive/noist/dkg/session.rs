use super::package::DKGPackage;
use crate::{
    hash::Hash,
    into::IntoScalar,
    noist::{core::vse, setup::setup::VSESetup},
    schnorr::{Authenticable, LiftScalar, Sighash},
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct DKGSession {
    index: u64,
    signatories: Vec<Point>,
    packages: HashMap<Point, Authenticable<DKGPackage>>,
}

impl DKGSession {
    pub fn new(index: u64, signatories: &Vec<Point>) -> Option<Self> {
        let mut signatories = signatories.clone();
        signatories.sort();

        let session = DKGSession {
            index,
            signatories,
            packages: HashMap::<Point, Authenticable<DKGPackage>>::new(),
        };

        Some(session)
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match serde_json::from_slice(bytes) {
            Ok(keymap) => Some(keymap),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match serde_json::to_vec(self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn signatories(&self) -> Vec<Point> {
        self.signatories
            .iter()
            .map(|secp_point| secp_point.clone())
            .collect()
    }

    pub fn auth_packages(&self) -> HashMap<Point, Authenticable<DKGPackage>> {
        self.packages
            .clone()
            .into_iter()
            .map(|(secp_key, auth_dkg_package)| (secp_key.clone(), auth_dkg_package))
            .collect()
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

        let signatories_: Vec<Point> = self
            .signatories
            .iter()
            .map(|secp_point| secp_point.clone())
            .collect();

        if !package.is_complete(&signatories_) {
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
        if !self.is_threshold_met() {
            return false;
        }

        for (signatory, auth_package) in self.packages.iter() {
            if auth_package.key() != signatory.serialize_xonly() {
                return false;
            }

            if !auth_package.authenticate() {
                return false;
            }

            let package = auth_package.object();

            let signatories_: Vec<Point> = self
                .signatories
                .iter()
                .map(|secp_point| secp_point.clone())
                .collect();

            if !package.is_complete(&signatories_) {
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

    /// Checks whether there are enough number of DKG packages based on the
    /// same threshold value required to produce a valid digital signature.
    pub fn is_threshold_met(&self) -> bool {
        let threshold = (self.signatories.len() / 2) + 1;
        self.packages.len() >= threshold
    }

    pub fn group_combined_hiding_point(&self) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages().iter() {
            combined_point = combined_point + package.hiding().constant_point()?;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn group_combined_pre_binding_point(&self) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages().iter() {
            combined_point = combined_point + package.binding().constant_point()?;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn group_combined_post_binding_point(
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

    pub fn group_combined_full_point(
        &self,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Point> {
        let hiding = self.group_combined_hiding_point()?;
        let binding = self.group_combined_post_binding_point(group_key, message)?;
        match hiding + binding {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn group_commitment_hash(&self) -> Option<[u8; 32]> {
        let mut preimage = Vec::<u8>::new();
        preimage.extend(self.index.to_be_bytes());

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

    pub fn signatory_combined_hiding_public(&self, signatory: Point) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages.iter() {
            let hiding_shares = package.object().hiding().shares();
            let share = hiding_shares.get(&signatory)?;
            combined_point = combined_point + share.0;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        }
    }

    pub fn signatory_combined_pre_binding_public(&self, signatory: Point) -> Option<Point> {
        let mut combined_point = MaybePoint::Infinity;

        for (_, package) in self.packages.iter() {
            let binding_shares = package.object().binding().shares();
            let share = binding_shares.get(&signatory)?;
            combined_point = combined_point + share.0;
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        }
    }

    pub fn signatory_combined_post_binding_public(
        &self,
        signatory: Point,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Point> {
        let binding_factors = self.binding_factors(group_key, message)?;
        let ordered_packages = self.ordered_packages();

        let mut combined_point = MaybePoint::Infinity;

        for (index, (_, package)) in ordered_packages.iter().enumerate() {
            let binding_factor = binding_factors[index].into_scalar().ok()?;

            let binding_shares = package.binding().shares();
            let share = binding_shares.get(&signatory)?;
            combined_point = combined_point + (share.0 * binding_factor);
        }

        match combined_point {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        }
    }

    pub fn signatory_combined_full_point(
        &self,
        signatory: Point,
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Point> {
        let hiding = self.signatory_combined_hiding_public(signatory)?;
        let binding = self.signatory_combined_post_binding_public(signatory, group_key, message)?;
        match hiding + binding {
            MaybePoint::Valid(point) => return Some(point),
            MaybePoint::Infinity => return None,
        };
    }

    pub fn signatory_combined_hiding_secret(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let signatory_secret = secret_key.into_scalar().ok()?.lift();
        let signatory_public = signatory_secret.base_point_mul();
        let mut combined_secret = MaybeScalar::Zero;

        for (signatory, package) in self.packages.iter() {
            let hiding_shares = package.object().hiding().shares();
            let encsec = hiding_shares.get(&signatory_public)?.1;
            let encrypting_key_secret =
                vse::encrypting_key_secret(signatory_secret, signatory.to_owned());
            let decsec = vse::decrypt(encsec, encrypting_key_secret).ok()?;

            combined_secret = combined_secret + decsec;
        }

        match combined_secret {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn signatory_combined_pre_binding_secret(&self, secret_key: [u8; 32]) -> Option<Scalar> {
        let signatory_secret = secret_key.into_scalar().ok()?.lift();
        let signatory_public = signatory_secret.base_point_mul();
        let mut combined_secret = MaybeScalar::Zero;

        for (signatory, package) in self.packages.iter() {
            let binding_shares = package.object().binding().shares();
            let encsec = binding_shares.get(&signatory_public)?.1;
            let encrypting_key_secret =
                vse::encrypting_key_secret(signatory_secret, signatory.to_owned());
            let decsec = vse::decrypt(encsec, encrypting_key_secret).ok()?;

            combined_secret = combined_secret + decsec;
        }

        match combined_secret {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn signatory_combined_post_binding_secret(
        &self,
        secret_key: [u8; 32],
        group_key: Option<[u8; 32]>,
        message: Option<[u8; 32]>,
    ) -> Option<Scalar> {
        let signatory_secret = secret_key.into_scalar().ok()?.lift();
        let signatory_public = signatory_secret.base_point_mul();
        let mut combined_secret = MaybeScalar::Zero;

        let binding_factors = self.binding_factors(group_key, message)?;

        for (index, (signatory, package)) in self.ordered_packages().iter().enumerate() {
            let binding_factor = binding_factors[index].into_scalar().ok()?;

            let binding_shares = package.binding().shares();
            let encsec = binding_shares.get(&signatory_public)?.1;
            let encrypting_key_secret =
                vse::encrypting_key_secret(signatory_secret, signatory.to_owned());
            let decsec = vse::decrypt(encsec, encrypting_key_secret).ok()?;

            combined_secret = combined_secret + (decsec * binding_factor);
        }

        match combined_secret {
            MaybeScalar::Valid(scalar) => return Some(scalar),
            MaybeScalar::Zero => return None,
        }
    }

    pub fn print(&self) {
        for (_, package) in self.ordered_packages().iter() {
            package.print();
        }
    }
}

impl Sighash for DKGSession {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage = Vec::<u8>::new();
        preimage.extend(self.index().to_be_bytes());

        let mut signatories = self.signatories();
        signatories.sort();

        for signatory in signatories.iter() {
            preimage.extend(signatory.serialize_xonly());
        }

        for (signatory, package) in self.ordered_packages().iter() {
            preimage.extend(signatory.serialize_xonly());
            preimage.extend(package.sighash());
        }

        preimage.hash(Some(crate::hash::HashTag::SighashAuthenticable))
    }
}
