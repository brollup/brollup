#![allow(dead_code)]

use crate::encoding::prefix::Prefix;
use crate::hash::{tagged_hash, HashTag};
use lazy_static::lazy_static;
use musig2::secp256k1::{self, Parity, PublicKey, Scalar, Secp256k1, XOnlyPublicKey};
use std::cmp::Ordering;
use std::vec;

type Bytes = Vec<u8>;

const LEAF_VERSION: u8 = 0xc0;

lazy_static! {
    static ref POINT_WITH_UNKNOWN_DISCRETE_LOGARITHM: Bytes = vec![
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0
    ];
}

pub trait P2TR {
    fn taproot(&self) -> Result<TapRoot, secp256k1::Error>;
    fn spk(&self) -> Result<Bytes, secp256k1::Error>;
}

#[derive(Clone)]
pub enum Branch {
    Leaf(TapLeaf),
    Branch(Box<TapBranch>),
}

#[derive(Clone)]
pub struct TapLeaf {
    leaf_version: u8,
    tap_script: Bytes,
}

impl TapLeaf {
    pub fn new(tap_script: Bytes) -> TapLeaf {
        TapLeaf {
            leaf_version: LEAF_VERSION,
            tap_script,
        }
    }

    pub fn new_version(tap_script: Bytes, leaf_version: u8) -> TapLeaf {
        TapLeaf {
            leaf_version,
            tap_script,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        hash_tap_leaf(&self.tap_script, self.leaf_version)
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Leaf(self.clone())
    }

    pub fn tap_script(&self) -> Bytes {
        self.tap_script.clone()
    }
}

#[derive(Clone)]
pub struct TapBranch {
    left_branch: Branch,
    right_branch: Branch,
}

impl TapBranch {
    pub fn new(first: Branch, second: Branch) -> TapBranch {
        let first_branch = match &first {
            Branch::Leaf(leaf) => leaf.hash(),
            Branch::Branch(branch) => branch.hash(),
        };

        let second_branch = match &second {
            Branch::Leaf(leaf) => leaf.hash(),
            Branch::Branch(branch) => branch.hash(),
        };

        match &first_branch.cmp(&second_branch) {
            Ordering::Less => TapBranch {
                left_branch: first,
                right_branch: second,
            },
            _ => TapBranch {
                left_branch: second,
                right_branch: first,
            },
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let left_branch = match &self.left_branch {
            Branch::Branch(branch) => branch.hash(),
            Branch::Leaf(leaf) => leaf.hash(),
        };

        let right_branch = match &self.right_branch {
            Branch::Branch(branch) => branch.hash(),
            Branch::Leaf(leaf) => leaf.hash(),
        };

        hash_tap_branch(left_branch, right_branch)
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Branch(Box::new(self.clone()))
    }
}

#[derive(Clone)]
pub struct TapRoot {
    inner_key: XOnlyPublicKey,
    tree: Option<TapTree>,
}

impl TapRoot {
    pub fn key_and_script_path_single(inner_key: XOnlyPublicKey, leaf: TapLeaf) -> TapRoot {
        TapRoot {
            inner_key,
            tree: Some(TapTree::new(vec![leaf])),
        }
    }

    pub fn key_and_script_path_multi(inner_key: XOnlyPublicKey, leaves: Vec<TapLeaf>) -> TapRoot {
        TapRoot {
            inner_key,
            tree: Some(TapTree::new(leaves)),
        }
    }

    pub fn key_path_only(inner_key: XOnlyPublicKey) -> TapRoot {
        TapRoot {
            inner_key,
            tree: None,
        }
    }

    pub fn script_path_only_single(leaf: TapLeaf) -> TapRoot {
        let inner_key = XOnlyPublicKey::from_slice(&POINT_WITH_UNKNOWN_DISCRETE_LOGARITHM).unwrap();
        TapRoot {
            inner_key,
            tree: Some(TapTree::new(vec![leaf])),
        }
    }

    pub fn script_path_only_multi(leaves: Vec<TapLeaf>) -> TapRoot {
        let inner_key = XOnlyPublicKey::from_slice(&POINT_WITH_UNKNOWN_DISCRETE_LOGARITHM).unwrap();
        TapRoot {
            inner_key,
            tree: Some(TapTree::new(leaves)),
        }
    }

    pub fn inner_key(&self) -> XOnlyPublicKey {
        self.inner_key
    }

    pub fn inner_key_lifted(&self) -> PublicKey {
        self.inner_key.public_key(Parity::Even)
    }

    pub fn uppermost_branch(&self) -> [u8; 32] {
        let uppermost_branch = match &self.tree {
            Some(tree) => match &tree.root {
                Branch::Leaf(leaf) => leaf.hash(),
                Branch::Branch(branch) => branch.hash(),
            },
            None => [0u8; 32],
        };
        uppermost_branch
    }

    pub fn tap_tweak(&self) -> [u8; 32] {
        let inner_key_bytes = self.inner_key.serialize();

        let uppermost_branch = self.uppermost_branch();

        hash_tap_tweak(inner_key_bytes, uppermost_branch)
    }

    pub fn tweaked_key(&self) -> Result<PublicKey, secp256k1::Error> {
        if let Some(_) = &self.tree {
            let scalar = Scalar::from_be_bytes(self.tap_tweak())
                .map_err(|_| secp256k1::Error::InvalidTweak)?;
            self.inner_key_lifted()
                .add_exp_tweak(&Secp256k1::new(), &scalar)
        } else {
            Ok(self.inner_key_lifted())
        }
    }

    pub fn tweaked_key_parity(&self) -> Result<Parity, secp256k1::Error> {
        let tweaked_key = self.tweaked_key()?;
        let (_, parity) = tweaked_key.x_only_public_key();
        Ok(parity)
    }

    pub fn tweaked_key_x_only(&self) -> Result<XOnlyPublicKey, secp256k1::Error> {
        let (x_only, _) = self.tweaked_key()?.x_only_public_key();
        Ok(x_only)
    }

    pub fn spk(&self) -> Result<Bytes, secp256k1::Error> {
        let mut spk: Bytes = vec![0x51, 0x20];
        let tweaked_key = self.tweaked_key()?;
        spk.extend(tweaked_key.x_only_public_key().0.serialize().to_vec());
        Ok(spk)
    }

    pub fn control_block(&self, index: usize) -> Result<ControlBlock, secp256k1::Error> {
        let path: Bytes = match &self.tree {
            Some(tree) => tree.path(index),
            None => return Err(secp256k1::Error::InvalidTweak),
        };

        let inner_key = self.inner_key();
        let parity = self.tweaked_key_parity()?;

        Ok(ControlBlock::new(inner_key, parity, path))
    }
    pub fn tree(&self) -> Option<TapTree> {
        self.tree.clone()
    }
}

#[derive(Clone)]
pub struct TapTree {
    leaves: Vec<TapLeaf>,
    root: Branch,
}

impl TapTree {
    pub fn new(leaves: Vec<TapLeaf>) -> TapTree {
        TapTree {
            leaves: leaves.clone(),
            root: tree_builder(&leaves, None).0,
        }
    }

    pub fn root(&self) -> [u8; 32] {
        match &self.root {
            Branch::Leaf(leaf) => leaf.hash(),
            Branch::Branch(branch) => branch.hash(),
        }
    }

    pub fn path(&self, index: usize) -> Bytes {
        // Given leaf index return the merkle path

        let path_vec = match tree_builder(&self.leaves, Some(index)).1 {
            Some(vec) => vec,
            None => panic!(),
        };
        path_vec
    }
    pub fn leaves(&self) -> Vec<TapLeaf> {
        self.leaves.clone()
    }
}

// tree_builder returns given a vector of leaves, the tree root,
// and optionally a merkle path corresponding to some leaf
pub fn tree_builder(leaves: &Vec<TapLeaf>, index: Option<usize>) -> (Branch, Option<Bytes>) {
    // Initialize path as empty
    let path: Bytes = Vec::<u8>::new();

    match leaves.len() {
        0 => panic!("TapTree must be initialized with at least one TapLeaf."),
        1 => {
            // If single-leaf
            let branch: Branch = leaves[0].into_branch();
            match &index {
                Some(_) => (branch, Some(path)),
                None => (branch, None),
            }
        }
        _ => {
            let mut path: Bytes = Vec::<u8>::new();
            let mut lookup: Option<Branch> = match index.clone() {
                Some(index) => Some(leaves[index].into_branch()),
                None => None,
            };

            // Number of TapTree levels is = log2(number of TapLeaves)
            let num_levels: u8 = (leaves.len() as f64).log2() as u8;

            let mut current_level: Vec<Branch> = Vec::new();
            let mut above_level: Vec<Branch> = Vec::new();

            // For each level of the TapTree
            for level in 0..(num_levels + 1) {
                // If it is the level zero, initialize current_level  with individual TapLeaves
                if level == 0 {
                    for i in 0..leaves.len() {
                        current_level.push(leaves[i].clone().into_branch());
                    }
                } else {
                    // If it is the level one or above, move above_level items into current_level, and reset above_level
                    current_level.clear();
                    current_level.extend(above_level.clone());
                    above_level.clear();
                }

                let mut iterator: usize = 0;
                let iterator_bound: usize = current_level.len();
                let operations: usize = match iterator_bound {
                    0 => panic!("This should not be the case."),
                    1 => 1,
                    _ => (iterator_bound / 2) + (iterator_bound % 2),
                };

                for _ in 0..operations {
                    match iterator_bound - iterator {
                        0 => panic!("This should not be the case."),
                        // last
                        1 => {
                            above_level.push(current_level[iterator].clone());
                            iterator += 1;
                        }
                        // two or more left in the current scope
                        _ => {
                            let first: Branch = current_level[iterator].clone();
                            let second: Branch = current_level[iterator + 1].clone();

                            let first_bytes = match &first {
                                Branch::Leaf(leaf) => leaf.hash(),
                                Branch::Branch(branch) => branch.hash(),
                            };

                            let second_bytes = match &second {
                                Branch::Leaf(leaf) => leaf.hash(),
                                Branch::Branch(branch) => branch.hash(),
                            };

                            let lookup_bytes = match &lookup {
                                Some(branch) => match branch {
                                    Branch::Leaf(leaf) => leaf.hash(),
                                    Branch::Branch(branch) => branch.hash(),
                                },
                                None => [0u8; 32],
                            };

                            // Lookup match?
                            let mut match_bool: bool = false;
                            if &first_bytes == &lookup_bytes {
                                path.extend(&second_bytes);
                                match_bool = true;
                            } else if &second_bytes == &lookup_bytes {
                                path.extend(&first_bytes);
                                match_bool = true;
                            }

                            let new_branch: TapBranch = TapBranch::new(first, second);

                            if match_bool {
                                lookup = Some(new_branch.into_branch());
                            }

                            above_level.push(new_branch.into_branch());
                            iterator += 2;
                        }
                    }
                }

                // At the end of each level, the iterator must have covered all branches of that level
                assert_eq!(iterator, iterator_bound);
            }

            // At the end, only the uppermost branch must be left
            assert_eq!(above_level.len(), 1);

            let branch: Branch = above_level[0].clone();

            match &index {
                Some(_) => (branch, Some(path)),
                None => (branch, None),
            }
        }
    }
}

pub struct ControlBlock {
    inner_key: XOnlyPublicKey,
    parity: Parity,
    leaf_version: u8,
    path: Bytes,
}

impl ControlBlock {
    pub fn new(inner_key: XOnlyPublicKey, parity: Parity, path: Bytes) -> ControlBlock {
        ControlBlock {
            inner_key,
            parity,
            leaf_version: LEAF_VERSION,
            path,
        }
    }

    pub fn to_vec(&self) -> Bytes {
        let mut vec: Bytes = Vec::<u8>::new();

        match self.parity {
            Parity::Even => vec.push(self.leaf_version),
            Parity::Odd => vec.push(self.leaf_version + 1),
        };

        vec.extend(self.inner_key.serialize().to_vec());
        vec.extend(self.path.clone());
        vec
    }
}

pub fn hash_tap_leaf(raw_script_bytes: &Bytes, version: u8) -> [u8; 32] {
    let mut data: Bytes = Vec::new();

    data.extend(&[version]);
    data.extend(raw_script_bytes.prefix_compact_size());

    tagged_hash(data, HashTag::TapLeaf)
}

pub fn hash_tap_branch(left_branch_bytes: [u8; 32], right_branch_bytes: [u8; 32]) -> [u8; 32] {
    let mut data: Bytes = Vec::new();

    data.extend(left_branch_bytes);
    data.extend(right_branch_bytes);

    tagged_hash(data, HashTag::TapBranch)
}

pub fn hash_tap_tweak(inner_key_bytes: [u8; 32], tweak_bytes: [u8; 32]) -> [u8; 32] {
    let mut data: Bytes = Vec::new();

    data.extend(inner_key_bytes);
    data.extend(tweak_bytes);

    tagged_hash(data, HashTag::TapTweak)
}
