use crate::{
    hash::Hash,
    into::{IntoPoint, IntoScalar},
    noist::{secret::secret_share_gen, setup::setup::VSESetup, vse},
    schnorr::{generate_secret, Bytes32},
};
use std::collections::HashMap;

type VSSCommitment = [u8; 33];
type NNSKey = [u8; 32];
type PublicShare = [u8; 33];
type SecretShareEnc = [u8; 32];

#[derive(Clone)]
pub struct DKGShareMap {
    signer: [u8; 32],
    vss_commitments: Vec<VSSCommitment>,
    shares: HashMap<NNSKey, (PublicShare, SecretShareEnc)>,
}

impl DKGShareMap {
    pub fn new(secret_key: [u8; 32], signatories: &Vec<[u8; 32]>) -> Option<Self> {
        let self_public = secret_key.secret_to_public()?;

        let polynomial_secret = {
            let mut preimage = Vec::<u8>::new();
            preimage.extend(generate_secret());
            preimage.extend(secret_key);
            preimage
                .hash(Some(crate::hash::HashTag::SecretKey))
                .into_scalar()
                .ok()?
        };

        let num_signatories = signatories.len() as u8;

        if num_signatories < 3 {
            return None;
        }

        let threshold = (num_signatories / 2) + 1;

        let (secret_shares, vss_points) =
            secret_share_gen(polynomial_secret, num_signatories, threshold).ok()?;

        let mut vss_commitments = Vec::<VSSCommitment>::new();
        {
            for vss_point in vss_points {
                vss_commitments.push(vss_point.serialize());
            }
        }

        let mut shares = HashMap::<NNSKey, (PublicShare, SecretShareEnc)>::new();

        {
            let mut signatories = signatories.clone();
            signatories.sort();

            for (index, signatory) in signatories.iter().enumerate() {
                let signatory_public_point = signatory.to_owned().into_point().ok()?;
                let self_secret_scalar = secret_key.into_scalar().ok()?;

                let secret_share = secret_shares[index].1;
                let public_share = secret_share.base_point_mul();

                let encrypting_key_secret =
                    vse::encrypting_key_secret(self_secret_scalar, signatory_public_point);
                let secret_share_enc = vse::encrypt(secret_share, encrypting_key_secret).ok()?;

                shares.insert(
                    signatory.to_owned(),
                    (public_share.serialize(), secret_share_enc.serialize()),
                );
            }
        }

        Some(DKGShareMap {
            signer: self_public,
            vss_commitments,
            shares,
        })
    }

    pub fn signer(&self) -> [u8; 32] {
        self.signer.clone()
    }

    pub fn vss_commitments(&self) -> Vec<VSSCommitment> {
        self.vss_commitments.clone()
    }

    pub fn shares(&self) -> HashMap<NNSKey, (PublicShare, SecretShareEnc)> {
        self.shares.clone()
    }

    pub fn ordered_shares(&self) -> Vec<(NNSKey, (PublicShare, SecretShareEnc))> {
        let mut maps: Vec<(NNSKey, (PublicShare, SecretShareEnc))> =
            self.shares().into_iter().collect();
        maps.sort_by(|a, b| a.0.cmp(&b.0));
        maps
    }

    pub fn share_by_key(&self, key: [u8; 32]) -> Option<(PublicShare, SecretShareEnc)> {
        let share = self.shares.get(&key)?;
        Some(share.to_owned())
    }

    pub fn share_by_index(&self, index: u8) -> Option<(PublicShare, SecretShareEnc)> {
        let ordered_shares = self.ordered_shares();
        let share = ordered_shares.get(index as usize)?;
        Some(share.1)
    }

    pub fn print(&self) {
        println!("VSS Commitments :");
        for (index, vss_commitment) in self.vss_commitments.iter().enumerate() {
            let str = match index {
                0 => format!("#0 -> {} (Constant Point)", hex::encode(vss_commitment)),
                _ => format!("#{} -> {}", index, hex::encode(vss_commitment)),
            };
            println!("{}", str);
        }

        println!("");

        println!("Shares :");
        for (index, (key, (pubshare, encsec))) in self.ordered_shares().iter().enumerate() {
            println!("#{} {}", index, hex::encode(key));
            println!("   pubshare: {}", hex::encode(pubshare));
            println!("   encsec: {}\n", hex::encode(encsec));
        }
    }

    pub fn vse_validate(&self, setup: VSESetup) -> bool {
        for (key, (pubshare, encsec)) in self.shares.iter() {
            let vse_key = match setup.vse_key(self.signer, key.to_owned()) {
                Some(vse_key) => vse_key,
                None => return false,
            };

            let encrypted_share_scalar = match encsec.into_scalar() {
                Ok(scalar) => scalar,
                Err(_) => return false,
            };

            let public_share_point = match pubshare.into_point() {
                Ok(point) => point,
                Err(_) => return false,
            };

            let encrypting_key_public = match vse_key.into_point() {
                Ok(point) => point,
                Err(_) => return false,
            };

            if !vse::verify(
                encrypted_share_scalar,
                public_share_point,
                encrypting_key_public,
            ) {
                return false;
            }
        }

        true
    }
}
