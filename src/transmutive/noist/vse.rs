use crate::{
    db,
    hash::{Hash, HashTag},
    into::{IntoPoint, IntoScalar},
    schnorr::{Authenticable, Bytes32, LiftScalar, Sighash},
    SIGNATORY_DB,
};
use secp::{MaybePoint, MaybeScalar, Point, Scalar};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn encrypting_key_secret(self_secret: [u8; 32], to_public: [u8; 32]) -> Option<[u8; 32]> {
    let self_secret = self_secret.into_scalar().ok()?;
    let to_public = to_public.into_point().ok()?;

    let shared_secret_point = self_secret * to_public;
    let shared_secret_point_bytes = shared_secret_point.serialize_uncompressed();
    let shared_secret_point_hash = (&shared_secret_point_bytes).hash(Some(HashTag::SecretKey));
    let shared_secret = match MaybeScalar::reduce_from(&shared_secret_point_hash) {
        MaybeScalar::Valid(scalar) => scalar.lift(),
        MaybeScalar::Zero => Scalar::reduce_from(&shared_secret_point_hash).lift(),
    };

    Some(shared_secret.serialize())
}

pub fn encrypting_key_public(self_secret: [u8; 32], to_public: [u8; 32]) -> Option<[u8; 32]> {
    let encrypting_key_secret = encrypting_key_secret(self_secret, to_public)?;
    encrypting_key_secret.secret_to_public()
}

pub fn encrypt(secret_to_encrypt: [u8; 32], encrypting_key_secret: [u8; 32]) -> Option<[u8; 32]> {
    let secret_to_encrypt = secret_to_encrypt.into_scalar().ok()?;
    let encrypting_key_secret = encrypting_key_secret.into_scalar().ok()?;

    match secret_to_encrypt + encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar.serialize()),
        MaybeScalar::Zero => None,
    }
}

pub fn decrypt(secret_to_decrypt: [u8; 32], encrypting_key_secret: [u8; 32]) -> Option<[u8; 32]> {
    let secret_to_decrypt = secret_to_decrypt.into_scalar().ok()?;
    let encrypting_key_secret = encrypting_key_secret.into_scalar().ok()?;

    match secret_to_decrypt - encrypting_key_secret {
        MaybeScalar::Valid(scalar) => Some(scalar.serialize()),
        MaybeScalar::Zero => None,
    }
}

pub fn verify(
    combined_scalar: [u8; 32],
    public_share_point: [u8; 33], // comperessed
    vse_public_key: [u8; 32],     // xonly
) -> bool {
    let combined_scalar = match Scalar::from_slice(&combined_scalar) {
        Ok(scalar) => scalar,
        Err(_) => return false,
    };

    let public_share_point = match Point::from_slice(&public_share_point) {
        Ok(point) => point,
        Err(_) => return false,
    };

    let vse_public_key = match Point::from_slice(&vse_public_key) {
        Ok(point) => point,
        Err(_) => return false,
    };
    let combined_point = combined_scalar.base_point_mul();

    combined_point
        == match public_share_point + vse_public_key {
            MaybePoint::Valid(point) => point,
            MaybePoint::Infinity => return false,
        }
}

type CorrespondantKey = [u8; 32];
type CorrespondantVSEKey = [u8; 32];
type VSEProof = Option<Vec<u8>>;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEKeyMap {
    key: [u8; 32],
    map: HashMap<CorrespondantKey, (CorrespondantVSEKey, VSEProof)>,
}

impl VSEKeyMap {
    pub fn new(self_secret: [u8; 32], list: &Vec<[u8; 32]>) -> Option<VSEKeyMap> {
        let self_public = self_secret.secret_to_public()?;

        let mut map = HashMap::<CorrespondantKey, (CorrespondantVSEKey, VSEProof)>::new();

        for to_public in list {
            if to_public != &self_public {
                let correspondant_vse_key = match encrypting_key_public(self_secret, *to_public) {
                    Some(vse_key) => vse_key,
                    None => return None,
                };

                map.insert(*to_public, (correspondant_vse_key, None));
            }
        }

        Some(VSEKeyMap {
            key: self_public,
            map,
        })
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(keymap) => Some(keymap),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn map(&self) -> HashMap<CorrespondantKey, (CorrespondantVSEKey, VSEProof)> {
        self.map.clone()
    }

    pub fn key(&self) -> [u8; 32] {
        self.key
    }

    pub fn map_list(&self) -> Vec<[u8; 32]> {
        let mut keys: Vec<[u8; 32]> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn full_list(&self) -> Vec<[u8; 32]> {
        let mut full_list = Vec::<[u8; 32]>::new();

        full_list.push(self.key());
        full_list.extend(self.map_list());
        full_list.sort();

        full_list
    }

    pub fn is_complete(&self, expected_list: &Vec<[u8; 32]>) -> bool {
        let expected_list = {
            let mut expected_list_ = expected_list.clone();
            expected_list_.sort();
            expected_list_
        };

        let full_list = self.full_list();

        if full_list.len() == expected_list.len() {
            for (index, key) in full_list.iter().enumerate() {
                if key != &expected_list[index] {
                    return false;
                }
            }
            return true;
        }

        false
    }

    pub fn vse_key(&self, correspondant: [u8; 32]) -> Option<[u8; 32]> {
        Some(self.map.get(&correspondant)?.0.to_owned())
    }
}

impl Sighash for VSEKeyMap {
    fn sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        preimage.extend(self.key());

        let mut maps: Vec<(CorrespondantKey, (CorrespondantVSEKey, VSEProof))> =
            self.map().into_iter().collect();
        maps.sort_by(|a, b| a.0.cmp(&b.0));

        for (signer_key, (vse_key, proof)) in maps.iter() {
            preimage.extend(signer_key);
            preimage.extend(vse_key);
            match proof {
                Some(proof) => {
                    preimage.push(0x01);
                    preimage.extend(proof)
                }
                None => preimage.push(0x00),
            }
        }

        preimage.hash(Some(HashTag::SighashAuthenticable))
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSESetup {
    signers: Vec<[u8; 32]>,
    maps: HashMap<[u8; 32], Authenticable<VSEKeyMap>>,
}

impl VSESetup {
    pub fn new(signers: &Vec<[u8; 32]>) -> VSESetup {
        VSESetup {
            signers: signers.clone(),
            maps: HashMap::<[u8; 32], Authenticable<VSEKeyMap>>::new(),
        }
    }

    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.signers.clone()
    }

    pub fn maps(&self) -> HashMap<[u8; 32], Authenticable<VSEKeyMap>> {
        self.maps.clone()
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(directory) => Some(directory),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub fn insert(&mut self, map: Authenticable<VSEKeyMap>) -> bool {
        if self.signers.contains(&map.object().key()) {
            if let None = self.maps.get(&map.key()) {
                if map.object().is_complete(&self.signers()) {
                    self.maps.insert(map.key(), map);
                }
                return true;
            }
        }
        false
    }

    pub fn auth_map(&self, signer: [u8; 32]) -> Option<Authenticable<VSEKeyMap>> {
        let map = self.maps().get(&signer)?.clone();
        Some(map)
    }

    pub fn map(&self, signer: [u8; 32]) -> Option<VSEKeyMap> {
        Some(self.auth_map(signer)?.object())
    }

    pub fn is_complete(&self) -> bool {
        if self.maps.len() != self.signers.len() {
            return false;
        }

        for (_, map) in self.maps.iter() {
            if !map.object().is_complete(&self.signers()) {
                return false;
            }
        }

        true
    }

    pub fn validate(&self) -> bool {
        // 0. Completeness
        if !self.is_complete() {
            return false;
        }

        for (key, map) in self.maps().iter() {
            // 1. Auth sigs
            {
                if !self.signers().contains(key) {
                    return false;
                }
                if key != &map.key() {
                    return false;
                }
                if !map.authenticate() {
                    return false;
                }
            }

            // 2. Sig matching.
            {
                let correspondants = map.object().map_list();

                for correspondant in correspondants.iter() {
                    let vse_key_ = match self.vse_key(key.to_owned(), correspondant.to_owned()) {
                        Some(key) => key,
                        None => return false,
                    };
                    let vse_key__ = match self.vse_key(correspondant.to_owned(), key.to_owned()) {
                        Some(key) => key,
                        None => return false,
                    };
                    if vse_key_ != vse_key__ {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn vse_key(&self, signer_1: [u8; 32], signer_2: [u8; 32]) -> Option<[u8; 32]> {
        for (key, map) in self.maps.iter() {
            if key == &signer_1 {
                if let Some(key) = map.object().vse_key(signer_2) {
                    return Some(key);
                }
            }
        }

        None
    }

    pub fn print(&self) {
        for (key, map) in self.maps().iter() {
            println!("{}", hex::encode(key));
            for triple in map.object().map().iter() {
                let proof = {
                    match triple.1 .1.clone() {
                        Some(proof) => hex::encode(proof),
                        None => "None".to_owned(),
                    }
                };
                println!(
                    "    {} -> vse_key: {} proof: {}",
                    hex::encode(triple.0),
                    hex::encode(triple.1 .0),
                    proof
                );
            }
            println!("");
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VSEDirectory {
    setups: HashMap<u64, VSESetup>,
}

impl VSEDirectory {
    pub async fn new(db: &SIGNATORY_DB) -> Option<Self> {
        let _db = db.lock().await;

        let directory = match _db.vse_directory_conn().get(db::VSE_DIRECTORY_PATH).ok()? {
            Some(data) => bincode::deserialize(&data).ok()?,
            None => VSEDirectory {
                setups: HashMap::<u64, VSESetup>::new(),
            },
        };

        Some(directory)
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        match bincode::deserialize(&bytes) {
            Ok(directory) => Some(directory),
            Err(_) => None,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    pub async fn setups(&self) -> HashMap<u64, VSESetup> {
        self.setups.clone()
    }

    pub async fn insert(&mut self, no: u64, setup: &VSESetup, db: &SIGNATORY_DB) -> bool {
        match self.setups.insert(no, setup.clone()) {
            Some(_) => return false,
            None => {
                self.prune();
                self.save(db).await
            }
        }
    }

    pub async fn save(&self, db: &SIGNATORY_DB) -> bool {
        let _db = db.lock().await;

        match _db
            .vse_directory_conn()
            .insert(db::VSE_DIRECTORY_PATH, self.serialize())
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub fn prune(&mut self) {
        if self.setups.len() > 3 {
            if let Some(&min_key) = self.setups.keys().min() {
                self.setups.remove(&min_key);
            }
        }
    }

    pub fn setup(&self, no: u64) -> Option<VSESetup> {
        Some(self.setups.get(&no)?.clone())
    }

    pub async fn print(&self) {
        for (batch_no, setup) in self.setups().await.iter() {
            println!("Setup #{} :", batch_no);
            setup.print();
            println!("");
        }
    }
}
