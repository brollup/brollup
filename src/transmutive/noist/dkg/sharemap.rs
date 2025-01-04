use std::collections::HashMap;

use crate::{
    into::{IntoPoint, IntoScalar},
    noist::{secret::secret_share_gen, vse},
};

type VSSCommitment = [u8; 33];
type NNSKey = [u8; 32];
type PublicShare = [u8; 33];
type SecretShareEnc = [u8; 32];

#[derive(Clone)]
pub struct DKGShareMap {
    vss_commitments: Vec<VSSCommitment>,
    shares: HashMap<NNSKey, (PublicShare, SecretShareEnc)>,
}

impl DKGShareMap {
    pub fn new(secret_key: [u8; 32], signatories: &Vec<[u8; 32]>) -> Option<Self> {
        let secret_key_scalar = secret_key.into_scalar().ok()?;
        let num_signatories = signatories.len() as u8;
        let threshold = num_signatories / 2;

        let (secret_shares, constant_point, variable_points) =
            secret_share_gen(secret_key_scalar, num_signatories, threshold).ok()?;

        let mut vss_commitments = Vec::<VSSCommitment>::new();
        {
            vss_commitments.push(constant_point.serialize());
            for variable_point in variable_points {
                vss_commitments.push(variable_point.serialize());
            }
        }

        let mut shares = HashMap::<NNSKey, (PublicShare, SecretShareEnc)>::new();

        {
            let mut signatories = signatories.clone();
            signatories.sort();

            for (index, signatory) in signatories.iter().enumerate() {
                let signatory_public_point = signatory.to_owned().into_point().ok()?;
                let self_secret_scalar = secret_key.into_scalar().ok()?;

                println!("index {}", index);

                println!(
                    "bu index sanirim {}",
                    hex::encode(secret_shares[index].0.serialize())
                );

                println!(
                    "bu share sanirim {}",
                    hex::encode(secret_shares[index].1.serialize())
                );

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

        None
    }

    pub fn vss_commitments(&self) -> Vec<VSSCommitment> {
        self.vss_commitments.clone()
    }

    pub fn shares(&self) -> HashMap<NNSKey, (PublicShare, SecretShareEnc)> {
        self.shares.clone()
    }
}
